package hardware;

import java.io.FileOutputStream;

public class M6809 {

    protected Memory mem;
    private int cl=0;

// 8bits registers
	private int A=0;
	private int B=0;
	private int DP=0;
	private int CC=0;
	
// 16bits registers
	private int X=0;
	private int Y=0;
	private int U=0;
	private int S=0;
	private int PC=0;
	private int D=0; // D is A+B
	
// fast CC bits (as ints) 
	private int res=0;
	private int m1=0;
	private int m2=0;
	private int sign=0;
	private int ovfl=0;
	private int h1=0;
	private int h2=0;
	private int ccrest=0;
	boolean wait=false;

    public M6809(Memory mem) {
	this.mem = mem;
	reset();
    }
    
    public int getPC() {
    	return PC;	
    }

    public void reset() {
    	
	PC=(mem.read(0xFFFE)<<8)|mem.read(0xFFFF);
	DP=0x00;
	S=0x8000;
	CC=0x00;

    	
    }

// recalculate A and B or D
private void CALCD() { D=(A<<8)|B; }
private void CALCAB() {	A=D>>8;B=D&0xFF; }



// basic 6809 addressing modes
private int IMMED8() {int M; M=PC;PC++; return M;}
private int IMMED16() {int M; M=PC;PC+=2; return M;}
private int DIREC() {int M;	M=(DP<<8)|mem.read(PC);PC++; return M;}
private int ETEND() {int M;	M=mem.read(PC)<<8;PC++;M|=mem.read(PC);PC++; return M;}
private int INDEXE() {
	int m;
	int m2;
	int M;
	m=mem.read(PC);PC++;
	if (m<0x80)
	{
		// effectue le complement a 2 sur la precision int
		int delta;
		if ((m&0x10)==0x10)
			delta=((-1>>5)<<5)|(m&0x1F);
		else
			delta=m&0x1F;
		int reg;
		switch (m&0xE0) {
			case 0x00 : reg=X;break;
			case 0x20 : reg=Y;break;
			case 0x40 : reg=U;break;
			case 0x60 : reg=S;break;
			default : return 0;	
		}
		cl++;
		return (reg+delta) & 0xFFFF;
	}
	switch (m) {
	case 0x80 : //i_d_P1_X
		M=X;
  		X=(X+1)&0xFFFF;cl+=2;
		return M;
  	case 0x81 : //i_d_P2_X
  		M=X;
  		X=(X+2)&0xFFFF;cl+=3;
		return M;
  	case 0x82 : //i_d_M1_X
  		X=(X-1)&0xFFFF;
  		M=X;cl+=2;
		return M;
  case 0x83 : //i_d_M2_X
  		X=(X-2)&0xFFFF;
  		M=X;cl+=3;
		return M;
  case 0x84 : //i_d_X
  		M=X;
  		return M;
  case 0x85 : //i_d_B_X
    	M=(X+signedChar(B))&0xFFFF;cl+=1;
    	return M;
  case 0x86 : //i_d_A_X;
    	M=(X+signedChar(A))&0xFFFF;cl+=1;
    	return M;
  case 0x87 : return 0; //i_undoc;	/* empty */
  case 0x88 :  //i_d_8_X;
  		m2=mem.read(PC);PC++;
  		M=(X+signedChar(m2))&0xFFFF;cl+=1;
  		return M;
  case 0x89 :  //i_d_16_X;
  		m2=(mem.read(PC)<<8)|mem.read(PC+1);PC+=2;
  		M=(X+signed16bits(m2))&0xFFFF;cl+=4;
  		return M;		
  case 0x8A : return 0; //i_undoc;	/* empty */
  case 0x8B :  //i_d_D_X;
	  	M=(X+signed16bits((A<<8)|B))&0xFFFF;
	  	cl+=4;
	  	return M;
  case 0x8C :  //i_d_PC8;
  case 0xAC :  //i_d_PC8;
  case 0xCC :  //i_d_PC8;
  case 0xEC :  //i_d_PC8;
  		m=mem.read(PC);PC=(PC+1)&0xFFFF;
  		M=(PC+signedChar(m))&0xFFFF;
  		cl++;
  		return M;
  case 0x8D :  //i_d_PC16;
  case 0xAD :  //i_d_PC16;
  case 0xCD :  //i_d_PC16;
  case 0xED :  //i_d_PC16;
  		M=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(PC+signed16bits(M))&0xFFFF;cl+=5;
  		return M;
  case 0x8E : return 0; //i_undoc;	/* empty */
  case 0x8F : return 0; //i_undoc;	/* empty */
  case 0x90 : return 0; //i_undoc;	/* empty */
  case 0x91 :  //i_i_P2_X;
    	M=(mem.read(X)<<8)|mem.read(X+1);
  		X=(X+2)&0xFFFF;cl+=6;
		return M;
  case 0x92 : return 0; //i_undoc;	/* empty */
  case 0x93 :  //i_i_M2_X;
	  	X=(X-2)&0xFFFF;
    	M=(mem.read(X)<<8)|mem.read(X+1);
		cl+=6;
		return M;
  case 0x94 :  //i_i_0_X;
    	M=(mem.read(X)<<8)|mem.read(X+1);
		cl+=3;
		return M;
  case 0x95 :  //i_i_B_X;
    	M=(X+signedChar(B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0x96 :  //i_i_A_X;
    	M=(X+signedChar(A))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0x97 : return 0; //i_undoc;	/* empty */
  case 0x98 :  //i_i_8_X;
    	m2=mem.read(PC);PC=(PC+1)&0xFFFF;
  		M=(X+signedChar(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0x99 :  //i_i_16_X;
    	m2=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(X+signed16bits(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
		return M;
  case 0x9A : return 0; //i_undoc;	/* empty */
  case 0x9B :  //i_i_D_X;
  		M=(X+signed16bits((A<<8)|B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
  		return M;
  case 0x9C :  //i_i_PC8;
  case 0xBC :  //i_i_PC8;
  case 0xDC :  //i_i_PC8;
  case 0xFC :  //i_i_PC8;
    	m2=mem.read(PC);PC=(PC+1)&0xFFFF;
  		M=(PC+signedChar(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);
		cl+=4;
		return M;
  case 0x9D :  //i_i_PC16;
  case 0xBD :  //i_i_PC16;
  case 0xDD :  //i_i_PC16;
  case 0xFD :  //i_i_PC16;
    	m2=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(PC+signed16bits(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);
		cl+=8;
		return M;
  case 0x9E : return 0; //i_undoc;	/* empty */
  case 0x9F :  //i_i_e16;
  case 0xBF :  //i_i_e16;
  case 0xDF :  //i_i_e16;
  case 0xFF :  //i_i_e16;
    	m2=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(mem.read(m2)<<8)|mem.read(m2+1);
		cl+=5;
		return M;
  // Y
  case 0xA0 :  //i_d_P1_Y;
		M=Y;
  		Y=(Y+1)&0xFFFF;cl+=2;
		return M;
  case 0xA1 :  //i_d_P2_Y;
  		M=Y;
  		Y=(Y+2)&0xFFFF;cl+=3;
		return M;
  case 0xA2 :  //i_d_M1_Y;
  		Y=(Y-1)&0xFFFF;
  		M=Y;cl+=2;
		return M;
  case 0xA3 :  //i_d_M2_Y;
  		Y=(Y-2)&0xFFFF;
  		M=Y;cl+=3;
		return M;
  case 0xA4 :  //i_d_Y;
  		M=Y;
  		return M;
  case 0xA5 :  //i_d_B_Y;
    	M=(Y+signedChar(B))&0xFFFF;cl+=1;
    	return M;
  case 0xA6 :  //i_d_A_Y;
    	M=(Y+signedChar(A))&0xFFFF;cl+=1;
		return M;
  case 0xA7 : return 0; //i_undoc;	/* empty */
  case 0xA8 :  //i_d_8_Y;
  		m2=mem.read(PC);PC++;
  		M=(Y+signedChar(m2))&0xFFFF;cl+=1;
  		return M;
  case 0xA9 :  //i_d_16_Y;
  		m2=(mem.read(PC)<<8)|mem.read(PC+1);PC+=2;
  		M=(Y+signed16bits(m2))&0xFFFF;cl+=4;
  		return M;		
  case 0xAA : return 0; //i_undoc;	/* empty */
  case 0xAB :  //i_d_D_Y;
	  	M=(Y+signed16bits((A<<8)|B))&0xFFFF;
	  	cl+=4;
	  	return M;
  case 0xAE : return 0; //i_undoc;	/* empty */
  case 0xAF : return 0; //i_undoc;	/* empty */
  case 0xB0 : return 0; //i_undoc;	/* empty */
  case 0xB1 :  //i_i_P2_Y;
    	M=(mem.read(Y)<<8)|mem.read(Y+1);
  		Y=(Y+2)&0xFFFF;cl+=6;
		return M;
  case 0xB2 : return 0; //i_undoc;	/* empty */
  case 0xB3 :  //i_i_M2_Y;
	  	Y=(Y-2)&0xFFFF;
    	M=(mem.read(Y)<<8)|mem.read(Y+1);
		cl+=6;
		return M;
  case 0xB4 :  //i_i_0_Y;
    	M=(mem.read(Y)<<8)|mem.read(Y+1);
		cl+=3;
		return M;
  case 0xB5 :  //i_i_B_Y;
    	M=(Y+signedChar(B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xB6 :  //i_i_A_Y;
    	M=(Y+signedChar(A))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xB7 : return 0; //i_undoc;	/* empty */
  case 0xB8 :  //i_i_8_Y;
    	m2=mem.read(PC);PC=(PC+1)&0xFFFF;
  		M=(Y+signedChar(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xB9 :  //i_i_16_Y;
    	m2=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(Y+signed16bits(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
		return M;
  case 0xBA : return 0; //i_undoc;	/* empty */
  case 0xBB :  //i_i_D_Y;
  		M=(Y+signed16bits((A<<8)|B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
  		return M;
  case 0xBE : return 0; //i_undoc;	/* empty */

  // U
  case 0xC0 :  //i_d_P1_U;
		M=U;
  		U=(U+1)&0xFFFF;cl+=2;
		return M;
  case 0xC1 :  //i_d_P2_U;
  		M=U;
  		U=(U+2)&0xFFFF;cl+=3;
		return M;
  case 0xC2 :  //i_d_M1_U;
  		U=(U-1)&0xFFFF;
  		M=U;cl+=2;
		return M;
  case 0xC3 :  //i_d_M2_U;
  		U=(U-2)&0xFFFF;
  		M=U;cl+=3;
		return M;
  case 0xC4 :  //i_d_U;
  		M=U;
  		return M;
  case 0xC5 :  //i_d_B_U;
    	M=(U+signedChar(B))&0xFFFF;cl+=1;
    	return M;
  case 0xC6 :  //i_d_A_U;
    	M=(U+signedChar(A))&0xFFFF;cl+=1;
		return M;
  case 0xC7 : return 0; //i_undoc;	/* empty */
  case 0xC8 :  //i_d_8_U;
  		m2=mem.read(PC);PC++;
  		M=(U+signedChar(m2))&0xFFFF;cl+=1;
  		return M;
  case 0xC9 :  //i_d_16_U;
  		m2=(mem.read(PC)<<8)|mem.read(PC+1);PC+=2;
  		M=(U+signed16bits(m2))&0xFFFF;cl+=4;
  		return M;		
  case 0xCA : return 0; //i_undoc;	/* empty */
  case 0xCB :  //i_d_D_U;
	  	M=(U+signed16bits((A<<8)|B))&0xFFFF;
	  	cl+=4;
	  	return M;
  case 0xCE : return 0; //i_undoc;	/* empty */
  case 0xCF : return 0; //i_undoc;	/* empty */
  case 0xD0 : return 0; //i_undoc;	/* empty */
  case 0xD1 :  //i_i_P2_U;
    	M=(mem.read(U)<<8)|mem.read(U+1);
  		U=(U+2)&0xFFFF;cl+=6;
		return M;
  case 0xD2 : return 0; //i_undoc;	/* empty */
  case 0xD3 :  //i_i_M2_U;
	  	U=(U-2)&0xFFFF;
    	M=(mem.read(U)<<8)|mem.read(U+1);
		cl+=6;
		return M;
  case 0xD4 :  //i_i_0_U;
    	M=(mem.read(U)<<8)|mem.read(U+1);
		cl+=3;
		return M;
  case 0xD5 :  //i_i_B_U;
    	M=(U+signedChar(B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xD6 :  //i_i_A_U;
    	M=(U+signedChar(A))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xD7 : return 0; //i_undoc;	/* empty */
  case 0xD8 :  //i_i_8_U;
    	m2=mem.read(PC);PC=(PC+1)&0xFFFF;
  		M=(U+signedChar(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xD9 :  //i_i_16_U;
    	m2=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(U+signed16bits(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
		return M;
  case 0xDA : return 0; //i_undoc;	/* empty */
  case 0xDB :  //i_i_D_U;
  		M=(U+signed16bits((A<<8)|B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
  		return M;
  case 0xDE : return 0; //i_undoc;	/* empty */

  // S
  case 0xE0 :  //i_d_P1_S;
		M=S;
  		S=(S+1)&0xFFFF;cl+=2;
		return M;
  case 0xE1 :  //i_d_P2_S;
  		M=S;
  		S=(S+2)&0xFFFF;cl+=3;
		return M;
  case 0xE2 :  //i_d_M1_S;
  		S=(S-1)&0xFFFF;
  		M=S;cl+=2;
		return M;
  case 0xE3 :  //i_d_M2_S;
  		S=(S-2)&0xFFFF;
  		M=S;cl+=3;
		return M;
  case 0xE4 :  //i_d_S;
  		M=S;
  		return M;
  case 0xE5 :  //i_d_B_S;
    	M=(S+signedChar(B))&0xFFFF;cl+=1;
    	return M;
  case 0xE6 :  //i_d_A_S;
    	M=(S+signedChar(A))&0xFFFF;cl+=1;
		return M;
  case 0xE7 : return 0; //i_undoc;	/* empty */
  case 0xE8 :  //i_d_8_S;
  		m2=mem.read(PC);PC++;
  		M=(S+signedChar(m2))&0xFFFF;cl+=1;
  		return M;
  case 0xE9 :  //i_d_16_S;
  		m2=(mem.read(PC)<<8)|mem.read(PC+1);PC+=2;
  		M=(S+signed16bits(m2))&0xFFFF;cl+=4;
  		return M;		
  case 0xEA : return 0; //i_undoc;	/* empty */
  case 0xEB :  //i_d_D_S;
	  	M=(S+signed16bits((A<<8)|B))&0xFFFF;
	  	cl+=4;
	  	return M;
  case 0xEE : return 0; //i_undoc;	/* empty */
  case 0xEF : return 0; //i_undoc;	/* empty */
  case 0xF0 : return 0; //i_undoc;	/* empty */
  case 0xF1 :  //i_i_P2_S;
    	M=(mem.read(S)<<8)|mem.read(S+1);
  		S=(S+2)&0xFFFF;cl+=6;
		return M;
  case 0xF2 : return 0; //i_undoc;	/* empty */
  case 0xF3 :  //i_i_M2_S;
	  	S=(S-2)&0xFFFF;
    	M=(mem.read(S)<<8)|mem.read(S+1);
		cl+=6;
		return M;
  case 0xF4 :  //i_i_0_S;
    	M=(mem.read(S)<<8)|mem.read(S+1);
		cl+=3;
		return M;
  case 0xF5 :  //i_i_B_S;
    	M=(S+signedChar(B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xF6 :  //i_i_A_S;
    	M=(S+signedChar(A))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xF7 : return 0; //i_undoc;	/* empty */
  case 0xF8 :  //i_i_8_S;
    	m2=mem.read(PC);PC=(PC+1)&0xFFFF;
  		M=(S+signedChar(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=4;
		return M;
  case 0xF9 :  //i_i_16_S;
    	m2=(mem.read(PC)<<8)|mem.read(PC+1);PC=(PC+2)&0xFFFF;
  		M=(S+signed16bits(m2))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
		return M;
  case 0xFA : return 0; //i_undoc;	/* empty */
  case 0xFB :  //i_i_D_S;
  		M=(S+signed16bits((A<<8)|B))&0xFFFF;
  		M=(mem.read(M)<<8)|mem.read(M+1);cl+=7;
  		return M;
  case 0xFE : return 0; //i_undoc;	/* empty */
}
	System.err.println("Indexed mode not implemented");
	return 0;}

// cc register recalculate from separate bits
private int getcc()
{
	if ((res&0xff)==0)
       CC=((((h1&15)+(h2&15))&16)<<1)
		  |((sign&0x80)>>4)
		  |(4)
		  |((	((~(m1^m2))&(m1^ovfl)) &0x80)>>6)
		  |((res&0x100)>>8)
		  |ccrest;
		else
       CC=((((h1&15)+(h2&15))&16)<<1)
		  |((sign&0x80)>>4)
		  |((	((~(m1^m2))&(m1^ovfl)) &0x80)>>6)
		  |((res&0x100)>>8)
		  |ccrest;
		
        return CC;
}

// calculate CC fast bits from CC register
private void setcc(int i)
{
  m1=m2=0;
  res=((i&1)<<8)|(4-(i&4));
  ovfl=(i&2)<<6;
  sign=(i&8)<<4;
  h1=h2=(i&32)>>2;
  ccrest=i&0xd0;
}

public int readCC() {
	getcc();
	return CC;
}

private int LOAD8(int ADR) {return mem.read(ADR);}

// LDx
private int LD8(int M,int c) {sign=mem.read(M);m1=ovfl;res=(res&0x100)|sign;cl+=c;return sign;}
private int LD16(int M,int c) {int R;R=( ((mem.read(M))<<8)|mem.read(M+1) ) & 0xFFFF;m1=ovfl;sign=R>>8;res=(res&0x100)|((sign|R)&0xFF);cl+=c; return R;}

// STx
private void ST8(int R,int adr,int c) {	mem.write(adr,R);m1=ovfl;sign=R;res=(res&0x100)|sign;cl+=c;}
private void ST16(int R,int adr,int c) { mem.write(adr,R>>8);mem.write(adr+1,R&0xFF);m1=ovfl;sign=R>>8;res=(res&0x100)|((sign|R)&0xFF);cl+=c;}

// LEA
private int LEA() {int R=INDEXE();res=(res&0x100)|((R|(R>>8))&0xFF);cl+=4;return R;}

// CLR
private void CLR(int M,int c) {mem.write(M,0);m1=~m2;sign=res=0;cl+=c;}

// EXG
private void EXG() {
	int r1;
	int r2;
	int m;
	int k;
	int l;
	m=mem.read(PC++);
	r1=(m&0xF0)>>4;
	r2=(m&0x0F);
	k=0; // only for javac
	l=0; // only for javac
	switch (r1) {
		case 0x00 : k=(A<<8)|B;break;
		case 0x01 : k=X;break;
		case 0x02 : k=Y;break;
		case 0x03 : k=U;break;
		case 0x04 : k=S;break;
		case 0x05 : k=PC;break;
		case 0x06 : k=getcc();break;
		case 0x07 : k=getcc();break;
		case 0x08 : k=A;break;
		case 0x09 : k=B;break;
		case 0x0A : k=getcc();break;
		case 0x0B : k=DP;break;
		case 0x0C : k=getcc();break;
		case 0x0D : k=getcc();break;
		case 0x0E : k=getcc();break;
		case 0x0F : k=getcc();break;
	} // of switch r1
	switch (r2) {
		case 0x00 : l=(A<<8)|B;A=(k>>8)&255;B=k&255;break;
		case 0x01 : l=X;X=k;break;
		case 0x02 : l=Y;Y=k;break;
		case 0x03 : l=U;U=k;break;
		case 0x04 : l=S;S=k;break;
		case 0x05 : l=PC;PC=k;break;
		case 0x06 : l=getcc();setcc(k);break;
		case 0x07 : l=getcc();setcc(k);break;
		case 0x08 : l=A;A=k&0xff;break;
		case 0x09 : l=B;B=k&0xff;break;
		case 0x0A : l=getcc();setcc(k);break;
		case 0x0B : l=DP;DP=k&0xff;break;
		case 0x0C : l=getcc();setcc(k);break;
		case 0x0D : l=getcc();setcc(k);break;
		case 0x0E : l=getcc();setcc(k);break;
		case 0x0F : l=getcc();setcc(k);break;
	} // of switch r2
	switch (r1) {
		case 0x00 : A=(l>>8)&255;B=l&255;;break;
		case 0x01 : X=l;break;
		case 0x02 : Y=l;break;
		case 0x03 : U=l;break;
		case 0x04 : S=l;break;
		case 0x05 : PC=l;break;
		case 0x06 : setcc(l);break;
		case 0x07 : setcc(l);break;
		case 0x08 : A=l&0xff;break;
		case 0x09 : B=l&0xff;break;
		case 0x0A : setcc(l);break;
		case 0x0B : DP=l&0xff;break;
		case 0x0C : setcc(l);break;
		case 0x0D : setcc(l);break;
		case 0x0E : setcc(l);break;
		case 0x0F : setcc(l);break;
	} // of second switch r1
	cl+=8;		
}

private void TFR() {
	int r1;
	int r2;
	int m;
	int k;
	m=mem.read(PC++);
	r1=(m&0xF0)>>4;
	r2=(m&0x0F);
	k=0; // only for javac
	switch (r1) {
		case 0x00 : k=(A<<8)|B;break;
		case 0x01 : k=X;break;
		case 0x02 : k=Y;break;
		case 0x03 : k=U;break;
		case 0x04 : k=S;break;
		case 0x05 : k=PC;break;
		case 0x06 : k=getcc();break;
		case 0x07 : k=getcc();break;
		case 0x08 : k=A;break;
		case 0x09 : k=B;break;
		case 0x0A : k=getcc();break;
		case 0x0B : k=DP;break;
		case 0x0C : k=getcc();break;
		case 0x0D : k=getcc();break;
		case 0x0E : k=getcc();break;
		case 0x0F : k=getcc();break;
	} // of switch r1
	switch (r2) {
		case 0x00 : A=(k>>8)&255;B=k&255;break;
		case 0x01 : X=k;break;
		case 0x02 : Y=k;break;
		case 0x03 : U=k;break;
		case 0x04 : S=k;break;
		case 0x05 : PC=k;break;
		case 0x06 : setcc(k);break;
		case 0x07 : setcc(k);break;
		case 0x08 : A=k&0xff;break;
		case 0x09 : B=k&0xff;break;
		case 0x0A : setcc(k);break;
		case 0x0B : DP=k&0xff;break;
		case 0x0C : setcc(k);break;
		case 0x0D : setcc(k);break;
		case 0x0E : setcc(k);break;
		case 0x0F : setcc(k);break;
	} // of switch r2
}

private void PSHS() {
	int m;
	m=mem.read(PC++);
  	if ((m & 0x80)!=0) {S--;mem.write(S,PC&0x00FF);S--;mem.write(S,PC>>8);cl+=2;}
  	if ((m & 0x40)!=0) {S--;mem.write(S,U&0x00FF);S--;mem.write(S,U>>8);cl+=2;}
  	if ((m & 0x20)!=0) {S--;mem.write(S,Y&0x00FF);S--;mem.write(S,Y>>8);cl+=2;}
  	if ((m & 0x10)!=0) {S--;mem.write(S,X&0x00FF);S--;mem.write(S,X>>8);cl+=2;}
  	if ((m & 0x08)!=0) {S--;mem.write(S,DP);cl++;}
  	if ((m & 0x04)!=0) {S--;mem.write(S,B);cl++;}
  	if ((m & 0x02)!=0) {S--;mem.write(S,A);cl++;}
  	if ((m & 0x01)!=0) {S--;getcc();mem.write(S,CC);cl++;}
  	cl+=5;
}
private void PSHU() {
	int m;
	m=mem.read(PC++);
  	if ((m & 0x80)!=0) {U--;mem.write(U,PC&0x00FF);U--;mem.write(U,PC>>8);cl+=2;}
  	if ((m & 0x40)!=0) {U--;mem.write(U,S&0x00FF);U--;mem.write(U,S>>8);cl+=2;}
  	if ((m & 0x20)!=0) {U--;mem.write(U,Y&0x00FF);U--;mem.write(U,Y>>8);cl+=2;}
  	if ((m & 0x10)!=0) {U--;mem.write(U,X&0x00FF);U--;mem.write(U,X>>8);cl+=2;}
  	if ((m & 0x08)!=0) {U--;mem.write(U,DP);cl++;}
  	if ((m & 0x04)!=0) {U--;mem.write(U,B);cl++;}
  	if ((m & 0x02)!=0) {U--;mem.write(U,A);cl++;}
  	if ((m & 0x01)!=0) {U--;getcc();mem.write(U,CC);cl++;}
  	cl+=5;
}

private void PULS() {
	int m;
	m=mem.read(PC++);
  	if ((m & 0x01)!=0) {CC=mem.read(S);setcc(CC);S++;cl++;}
  	if ((m & 0x02)!=0) {A=mem.read(S);S++;cl++;}
  	if ((m & 0x04)!=0) {B=mem.read(S);S++;cl++;}
  	if ((m & 0x08)!=0) {DP=mem.read(S);S++;cl++;}
  	if ((m & 0x10)!=0) {X=(mem.read(S)<<8)|mem.read(S+1);S+=2;cl+=2;}
  	if ((m & 0x20)!=0) {Y=(mem.read(S)<<8)|mem.read(S+1);S+=2;cl+=2;}
  	if ((m & 0x40)!=0) {U=(mem.read(S)<<8)|mem.read(S+1);S+=2;cl+=2;}
  	if ((m & 0x80)!=0) {PC=(mem.read(S)<<8)|mem.read(S+1);S+=2;cl+=2;}
  	cl+=5;
}

private void PULU() {
	int m;
	m=mem.read(PC++);
  	if ((m & 0x01)!=0) {CC=mem.read(U);setcc(CC);U++;cl++;}
  	if ((m & 0x02)!=0) {A=mem.read(U);U++;cl++;}
  	if ((m & 0x04)!=0) {B=mem.read(U);U++;cl++;}
  	if ((m & 0x08)!=0) {DP=mem.read(U);U++;cl++;}
  	if ((m & 0x10)!=0) {X=(mem.read(U)<<8)|mem.read(U+1);U+=2;cl+=2;}
  	if ((m & 0x20)!=0) {Y=(mem.read(U)<<8)|mem.read(U+1);U+=2;cl+=2;}
  	if ((m & 0x40)!=0) {S=(mem.read(U)<<8)|mem.read(U+1);U+=2;cl+=2;}
  	if ((m & 0x80)!=0) {PC=(mem.read(U)<<8)|mem.read(U+1);U+=2;cl+=2;}
  	cl+=5;
}

private void INCA() {
	m1=A;m2=0;A=(A+1)&0xFF;
	ovfl=sign=A;
	res=(res&0x100)|sign;
	cl+=2;
}

private void INCB() {
	m1=B;m2=0;B=(B+1)&0xFF;
	ovfl=sign=B;
	res=(res&0x100)|sign;
	cl+=2;
}

private void INC(int adr,int c) {
	int val;val=mem.read(adr);
	m1=val;m2=0;
	val++;
	mem.write(adr,val);
	ovfl=sign=val&0xFF;
	res=(res&0x100)|sign;
	cl+=c;
}

// DEC
private void DECA() {
	m1=A;m2=0x80;
	A=(A-1)&0xFF;
	ovfl=sign=A;
	res=(res&0x100)|sign;
	cl+=2;
}

private void DECB() {
	m1=B;m2=0x80;
	B=(B-1)&0xFF;
	ovfl=sign=B;
	res=(res&0x100)|sign;
	cl+=2;
}

private void DEC(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=val;m2=0x80;
	val--;
	mem.write(adr,val);
	ovfl=sign=val&0xFF;
	res=(res&0x100)|sign;
	cl+=c;
}

private void BIT(int R,int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=ovfl;sign=R&val;
	res=(res&0x100)|sign;
	cl+=c;
}

private void CMP8(int R,int adr,int c) { 
 	int val;
 	val=mem.read(adr);
 	m1=R;m2=-val;
 	ovfl=res=sign=R-val;
 	cl+=c;
}

private void CMP16(int R,int adr,int c) {
	int val;
	val=(mem.read(adr)<<8)|mem.read(adr+1);
	m1=R>>8;m2=(-val)>>8;
	ovfl=res=sign=((R-val)>>8) & 0xFFFFFF;
	res|=(R-val)&0xFF;
	cl+=c;
}

// TST
private void TSTAi() {m1=ovfl;sign=A;res=(res&0x100)|sign;cl+=2;}
private void TSTBi() {m1=ovfl;sign=B;res=(res&0x100)|sign;cl+=2;}

private void TST(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=~m2;sign=val;res=(res&0x100)|sign;
	cl+=c;
}

private void ANDA(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=ovfl;A&=val;sign=A;
	res=(res&0x100)|sign;
	cl+=c;
}

private void ANDB(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=ovfl;B&=val;sign=B;
	res=(res&0x100)|sign;
	cl+=c;
}

private void ANDCC(int adr,int c) {
	int val;
	val=mem.read(adr);
//	getcc();
	CC&=val;setcc(CC);
	cl+=c;
}

private void ORA(int adr,int c) { 
	int	val;
	val=mem.read(adr);
	m1=ovfl;
	A|=val;sign=A;
	res=(res&0x100)|sign;
	cl+=c;
}

private void ORB(int adr,int c) { 
	int	val;
	val=mem.read(adr);
	m1=ovfl;
	B|=val;sign=B;
	res=(res&0x100)|sign;
	cl+=c;
}

private void ORCC(int adr,int c) { 
	int	val;
	val=mem.read(adr);
	getcc();
	CC|=val;setcc(CC);
	cl+=c;
}

private void EORA(int adr,int c) { 
	int	val;
	val=mem.read(adr);
	m1=ovfl;
	A^=val;sign=A;
	res=(res&0x100)|sign;
	cl+=c;
}

private void EORB(int adr,int c) { 
	int	val;
	val=mem.read(adr);
	m1=ovfl;
	B^=val;sign=B;
	res=(res&0x100)|sign;
	cl+=c;
}

private void COMA() {
	m1=ovfl;
	A=(~A)&0xFF;
	sign=A;
	res=sign|0x100;
	cl+=2;
}

private void COMB() {
	m1=ovfl;
	B=(~B)&0xFF;
	sign=B;
	res=sign|0x100;
	cl+=2;
}

private void COM(int adr,int c) {
	int val;
	val=mem.read(adr);m1=~m2;
	val=(~val)&0xFF;
	mem.write(adr,val);
	sign=val;
	res=sign|0x100;
	cl+=c;
}

private void NEGA() {
	m1=A;
	m2=-A;
	A=-A;
	ovfl=res=sign=A;
	A&=0xFF;
	cl+=2;
}

private void NEGB() {
	m1=B;
	m2=-B;
	B=-B;
	ovfl=res=sign=B;
	B&=0xFF;
	cl+=2;
}

private void NEG(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=val;m2=-val;val=-val;
	mem.write(adr,val);
	ovfl=res=sign=val;
	cl+=c;
}

private void ABX() {
	X=(X+B)&0xFFFF;
	cl+=3;
}

private void ADDA(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=h1=A;
	m2=h2=val;
	A+=val;
	ovfl=res=sign=A;A&=0xFF;
	cl+=c;
}

private void ADDB(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=h1=B;
	m2=h2=val;
	B+=val;
	ovfl=res=sign=B;B&=0xFF;
	cl+=c;
}

private void ADDD(int adr,int c) {
	int val;val=(mem.read(adr)<<8)|mem.read(adr+1);
	m1=A;m2=val>>8;
	D=(A<<8)+B+val;
	A=D>>8;B=D&0xFF;
	ovfl=res=sign=A;
	res|=B;A&=0xFF;
	cl+=c;
}

private void ADCA(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=h1=A;m2=val;
	h2=val+((res&0x100)>>8);
	A+=h2;ovfl=res=sign=A;A&=0xFF;
	cl+=c;
}

private void ADCB(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=h1=B;m2=val;
	h2=val+((res&0x100)>>8);
	B+=h2;ovfl=res=sign=B;B&=0xFF;
	cl+=c;
}

private void MUL() {
	int k;
  	k=A*B;
  	A=(k>>8)&0xFF;
  	B=k&0xFF;
  	res=((B&0x80)<<1)|((k|(k>>8))&0xFF);
  	cl+=11;
}

private void SBCA(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=A;m2=-val;
	A-=val+((res&0x100)>>8);
	ovfl=res=sign=A;
	A&=0xFF;
	cl+=c;
}

private void SBCB(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=B;m2=-val;
	B-=val+((res&0x100)>>8);
	ovfl=res=sign=B;
	B&=0xFF;
	cl+=c;
}

private void SUBA(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=A;m2=-val;
	A-=val;
	ovfl=res=sign=A;
	A&=0xFF;
	cl+=c;
}

private void SUBB(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=B;m2=-val;
	B-=val;
	ovfl=res=sign=B;
	B&=0xFF;
	cl+=c;
}

private void SUBD(int adr,int c) {
	int val;
	val=(mem.read(adr)<<8)|mem.read(adr+1);
	m1=A;
	m2=(-val)>>8;
	D=(A<<8)+B-val;
	A=D>>8;
	B=D&0xFF;
	ovfl=res=sign=A;
	res|=B;
	A&=0xFF;
	cl+=c;
}

private void SEX() {
  if ((B&0x80)==0x80) A=0xFF;
  else A=0;
  sign=B;
  res=(res&0x100)|sign;
  cl+=2;
}

private void ASLA() {
	m1=m2=A;
	A<<=1;
	ovfl=sign=res=A;
	A&=0xFF;
	cl+=2;
}

private void ASLB() {
	m1=m2=B;
	B<<=1;
	ovfl=sign=res=B;
	B&=0xFF;
	cl+=2;
}

private void ASL(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=m2=val;
	val<<=1;
	mem.write(adr,val);
	ovfl=sign=res=val;
	cl+=c;
}

private void ASRA() {
	res=(A&1)<<8;
	A=(A>>1)|(A&0x80);
	sign=A;
	res|=sign;
	cl+=2;
}

private void ASRB() {
	res=(B&1)<<8;
	B=(B>>1)|(B&0x80);
	sign=B;
	res|=sign;
	cl+=2;
}

private void ASR(int adr,int c) {
	int val;
	val=mem.read(adr);
	res=(val&1)<<8;
	val=(val>>1)|(val&0x80);
	mem.write(adr,val);
	sign=val;
	res|=sign;
	cl+=c;
}

private void LSRA() {
	res=(A&1)<<8;
	A=(A>>1);
	sign=0;res|=A;
	cl+=2;
}

private void LSRB() {
	res=(B&1)<<8;
	B=(B>>1);
	sign=0;res|=B;
	cl+=2;
}

private void LSR(int adr,int c) {
	int val;
	val=mem.read(adr);
	res=(val&1)<<8;
	val=(val>>1);
	mem.write(adr,val);
	sign=0;res|=val;
	cl+=c;
}

private void ROLA() {
	m1=m2=A;
	A=(A<<1)|((res&0x100)>>8);
	ovfl=sign=res=A;
	A&=0xFF;
	cl+=2;
}

private void ROLB() {
	m1=m2=B;
	B=(B<<1)|((res&0x100)>>8);
	ovfl=sign=res=B;
	B&=0xFF;
	cl+=2;
}

private void ROL(int adr,int c) {
	int val;
	val=mem.read(adr);
	m1=m2=val;
	val=(val<<1)|((res&0x100)>>8);
	mem.write(adr,val);
	ovfl=sign=res=val;
	cl+=c;
}

private void RORA() {
	int i;
	i=A;
	A=(A|(res&0x100))>>1;
	sign=A;
	res=((i&1)<<8)|sign;
	cl+=2;
}

private void RORB() {
	int i;
	i=B;
	B=(B|(res&0x100))>>1;
	sign=B;
	res=((i&1)<<8)|sign;
	cl+=2;
}

private void ROR(int adr,int c) {
	int i;
	int val;
	i=val=mem.read(adr);
	val=(val|(res&0x100))>>1;
	mem.write(adr,val);
	sign=val;
	res=((i&1)<<8)|sign;
	cl+=c;
}

private void BRA() {
	int m;
  	m=mem.read(PC++);
  	PC+=signedChar(m);
  	cl+=3;
}

private void LBRA() {
	int m;
  	int off;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	PC=(PC+off)&0xFFFF;
  	cl+=5;
}

private void JMPd() {
	int m;
  	m=mem.read(PC++);
  	PC=(DP<<8)|m;
  	cl+=3;
}

private void JMPe() {
	int adr;
  	adr=ETEND();
  	PC=adr;
  	cl+=4;
}

private void JMPx() {
	int adr;
  	adr=INDEXE();
  	PC=adr;
  	cl+=3;
}

private void BSR() {
	int m;
  	m=mem.read(PC++);
  	S--;mem.write(S,PC&0x00FF);
  	S--;mem.write(S,PC>>8);
  	PC+=signedChar(m);
  	cl+=7;
}

private void LBSR() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	S--;mem.write(S,PC&0x00FF);
  	S--;mem.write(S,PC>>8);
  	PC=(PC+off)&0xFFFF;
  	cl+=9;
}

private void JSRd() {
	int m;
  	m=mem.read(PC++);
  	S--;mem.write(S,PC&0x00FF);
  	S--;mem.write(S,PC>>8);
  	PC=(DP<<8)|m;
  	cl+=7;
}

private void JSRe() {
	int adr;	
  	adr=ETEND();
  	S--;mem.write(S,PC&0x00FF);
  	S--;mem.write(S,PC>>8);
  	PC=adr;
  	cl+=8;
}

private void JSRx() {
	int adr;	
  	adr=INDEXE();
  	S--;mem.write(S,PC&0x00FF);
  	S--;mem.write(S,PC>>8);
  	PC=adr;
  	cl+=7;
}

private void BRN() {
	int m;
  	m=mem.read(PC++);
  	cl+=3;
}

private void LBRN() {
	int m;
  	m=mem.read(PC++);
  	m=mem.read(PC++);
  	cl+=5;
}

private void NOP() {
  	cl+=2;
}

private void RTS()
{
  PC=(mem.read(S)<<8)|mem.read(S+1);S+=2;
  cl+=5;
}

/* Branchements conditionnels */

private void BCC() {
	int m;
  	m=mem.read(PC++);
  	if ((res&0x100)!=0x100) PC+=signedChar(m);
  	cl+=3;
}

private void LBCC() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ((res&0x100)!=0x100) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BCS() {
	int m;
  	m=mem.read(PC++);
  	if ((res&0x100)==0x100) PC+=signedChar(m);
  	cl+=3;
}

private void LBCS() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ((res&0x100)==0x100) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BEQ() {
	int m;
  	m=mem.read(PC++);
  	if ((res&0xff)==0x00) PC+=signedChar(m);
  	cl+=3;
}

private void LBEQ() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ((res&0xff)==0x00) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=6;
}

private void BNE() {
	int m;
  	m=mem.read(PC++);
  	if ((res&0xff)!=0) PC+=signedChar(m);
  	cl+=3;
}

private void LBNE() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ((res&0xff)!=0) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BGE() {
	int m;
  	m=mem.read(PC++);
  	if (((sign^((~(m1^m2))&(m1^ovfl)))&0x80)==0) PC+=signedChar(m);
  	cl+=3;
}

private void LBGE() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if (((sign^((~(m1^m2))&(m1^ovfl)))&0x80)==0) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BLE() {
	int m;
  	m=mem.read(PC++);
  	if ( ((res&0xff)==0)
   	     ||(((sign^((~(m1^m2))&(m1^ovfl)))&0x80)!=0) ) PC+=signedChar(m);
  	cl+=3;
}

private void LBLE() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ( ((res&0xff)==0)
   	     ||(((sign^((~(m1^m2))&(m1^ovfl)))&0x80)!=0) ) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BLS() {
	int m;
  	m=mem.read(PC++);
  	if (((res&0x100)!=0)||((res&0xff)==0)) PC+=signedChar(m);
  	cl+=3;
}

private void LBLS() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if (((res&0x100)!=0)||((res&0xff)==0)) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BGT() {
	int m;
  	m=mem.read(PC++);
  	if ( ((res&0xff)!=0)
   	     &&(((sign^((~(m1^m2))&(m1^ovfl)))&0x80)==0) ) PC+=signedChar(m);
  	cl+=3;
}

private void LBGT() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ( ((res&0xff)!=0)
   	     &&(((sign^((~(m1^m2))&(m1^ovfl)))&0x80)==0) ) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BLT() {
	int m;
  	m=mem.read(PC++);
  	if (((sign^((~(m1^m2))&(m1^ovfl)))&0x80)!=0) PC+=signedChar(m);
  	cl+=3;
}

private void LBLT() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if (((sign^((~(m1^m2))&(m1^ovfl)))&0x80)!=0)  {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BHI() {
	int m;
  	m=mem.read(PC++);
  	if (((res&0x100)==0)&&((res&0xff)!=0)) PC+=signedChar(m);
  	cl+=3;
}

private void LBHI() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if (((res&0x100)==0)&&((res&0xff)!=0)) {PC=(PC+off)&0xFFFF;cl+=6;}
  	cl+=5;
}

private void BMI() {
	int m; 
  	m=mem.read(PC++);
  	if ((sign&0x80)!=0) PC+=signedChar(m);
  	cl+=3;
}

private void LBMI() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ((sign&0x80)!=0) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BPL() {
	int m;
  	m=mem.read(PC++);
  	if ((sign&0x80)==0) PC+=signedChar(m);
  	cl+=3;
}

private void LBPL() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ((sign&0x80)==0) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BVS() {
	int m;
  	m=mem.read(PC++);
  	if ( (((m1^m2)&0x80)==0)&&(((m1^ovfl)&0x80)!=0) ) PC+=signedChar(m);
  	cl+=3;
}

private void LBVS() {
  	int off;
  	int m;
  	m=mem.read(PC++);off=m<<8;
  	m=mem.read(PC++);off|=m;
  	if ( (((m1^m2)&0x80)==0)&&(((m1^ovfl)&0x80)!=0) ) {PC=(PC+off)&0xFFFF;cl+=6;}
  	else cl+=5;
}

private void BVC() {
	int m;
  	m=mem.read(PC++);
  	if ( (((m1^m2)&0x80)!=0)||(((m1^ovfl)&0x80)==0) ) PC+=signedChar(m);
  	cl+=3;
}

private void LBVC() {
  int off;
  int m;
  m=mem.read(PC++);off=m<<8;
  m=mem.read(PC++);off|=m;
  if ( (((m1^m2)&0x80)!=0) || ( ((m1^ovfl)&0x80)==0) ) {PC=(PC+off)&0xFFFF;cl+=6;}
  else cl+=5;
}

private void SWI() {
  getcc();
  CC|=0x80; /* bit E … 1 */
  setcc(CC);
  S--;mem.write(S,PC&0x00FF);S--;mem.write(S,PC>>8);
  S--;mem.write(S,U&0x00FF);S--;mem.write(S,U>>8);
  S--;mem.write(S,Y&0x00FF);S--;mem.write(S,Y>>8);
  S--;mem.write(S,X&0x00FF);S--;mem.write(S,X>>8);
  S--;mem.write(S,DP);
  S--;mem.write(S,B);
  S--;mem.write(S,A);
  S--;mem.write(S,CC);
  PC=(mem.read(0xFFFA)<<8)|mem.read(0xFFFB);
  cl+=19;
}

private void RTI()
{
  CC=mem.read(S);setcc(CC);S++;
  if ((CC&0x80)==0x80)
  {
    A=mem.read(S);S++;
    B=mem.read(S);S++;
    DP=mem.read(S);S++;
    X=(mem.read(S)<<8)|mem.read(S+1);S+=2;
    Y=(mem.read(S)<<8)|mem.read(S+1);S+=2;
    U=(mem.read(S)<<8)|mem.read(S+1);S+=2;
    cl+=15;
  }
  else cl+=6;

  PC=(mem.read(S)<<8)|mem.read(S+1);S+=2;
}

public void IRQ() {
	/* mise … 1 du bit E sur le CC */
    getcc();
	CC|=0x80;
    setcc(CC);
	S--;mem.write(S,PC&0x00FF);S--;mem.write(S,PC>>8);
	S--;mem.write(S,U&0x00FF);S--;mem.write(S,U>>8);
	S--;mem.write(S,Y&0x00FF);S--;mem.write(S,Y>>8);
	S--;mem.write(S,X&0x00FF);S--;mem.write(S,X>>8);
	S--;mem.write(S,DP);
	S--;mem.write(S,B);
	S--;mem.write(S,A);
	S--;mem.write(S,CC);
	PC=(mem.read(0xFFF8)<<8)|mem.read(0xFFF9);
	CC|=0x10;
	setcc(CC);
	cl+=19;
}

private void DAA() {
   int     i=A+(res&0x100);
   if (((A&15)>9)||((h1&15)+(h2&15)>15)) i+=6;
   if (i>0x99) i+=0x60;
   res=sign=i;
   A=i&255;
   cl+=2;
}

private void CWAI() {
  getcc();
  CC&=mem.read(PC);
  setcc(CC);
  PC++;
  cl+=20;
  wait=true;
}

public int FetchUntil(int clock) {
	while (cl<clock) Fetch();
	cl-=clock;
	return (cl);
}

public void Fetch() {
	int opcode = mem.read(PC++);

	switch (opcode) {
	case 0x02 : mem.periph(PC,S);break;
	// LDA
	case 0x86: A=LD8(IMMED8(),2); break;
	case 0x96: A=LD8(DIREC(),4); break;
	case 0xB6: A=LD8(ETEND(),5); break;
	case 0xA6: A=LD8(INDEXE(),4); break;	
	
	// LDB
	case 0xC6: B=LD8(IMMED8(),2); break;
	case 0xD6: B=LD8(DIREC(),4); break;
	case 0xF6: B=LD8(ETEND(),5); break;
	case 0xE6: B=LD8(INDEXE(),4); break;
	
	// LDD
	case 0xCC : D=LD16(IMMED16(),3);CALCAB();break;
	case 0xDC : D=LD16(DIREC(),5);CALCAB();break;
	case 0xFC : D=LD16(ETEND(),6);CALCAB();break;
	case 0xEC : D=LD16(INDEXE(),5);CALCAB();break;

	// LDU
	case 0xCE : U=LD16(IMMED16(),3);break;
	case 0xDE : U=LD16(DIREC(),5);break;
	case 0xFE : U=LD16(ETEND(),6);break;
	case 0xEE : U=LD16(INDEXE(),5);break;

	
	// LDX
	case 0x8E : X=LD16(IMMED16(),3);break;
	case 0x9E : X=LD16(DIREC(),5);break;
	case 0xBE : X=LD16(ETEND(),6);break;
	case 0xAE : X=LD16(INDEXE(),5);break;

	// STA 
	case 0x97 : ST8(A,DIREC(),4);break;
	case 0xB7 : ST8(A,ETEND(),5);break;
	case 0xA7 : ST8(A,INDEXE(),4);break;

// STB
	case 0xD7 : ST8(B,DIREC(),4);break;
	case 0xF7 : ST8(B,ETEND(),5);break;
	case 0xE7 : ST8(B,INDEXE(),4);break;

// STD
	case 0xDD : CALCD();ST16(D,DIREC(),5);break;
	case 0xFD : CALCD();ST16(D,ETEND(),6);break;
	case 0xED : CALCD();ST16(D,INDEXE(),6);break;

// STU
	case 0xDF : ST16(U,DIREC(),5);break;
	case 0xFF : ST16(U,ETEND(),6);break;
	case 0xEF : ST16(U,INDEXE(),5);break;

// STX
	case 0x9F : ST16(X,DIREC(),5);break;
	case 0xBF : ST16(X,ETEND(),6);break;
	case 0xAF : ST16(X,INDEXE(),5);break;

// LEAS
	case 0x32 : S=INDEXE();break;
// LEAU
	case 0x33 : U=INDEXE();break;
// LEAX
	case 0x30 : X=LEA();break;
// LEAY
	case 0x31 : Y=LEA();break;

// CLRA
	case 0x4F : A=0;m1=ovfl;sign=res=0;cl+=2;break;
// CLRB
	case 0x5F : B=0;m1=ovfl;sign=res=0;cl+=2;break;
// CLR
	case 0x0F : CLR(DIREC(),6);break;
	case 0x7F : CLR(ETEND(),7);break;
	case 0x6F : CLR(INDEXE(),6);break;
	
// EXG
	case 0x1E : EXG();break;
	
// TFR
	case 0x1F : TFR();break;

// PSH/PUL
	case 0x34 : PSHS();break;
	case 0x36 : PSHU();break;
	case 0x35 : PULS();break;
	case 0x37 : PULU();break;

// INC
	case 0x4C : INCA();break;
	case 0x5C : INCB();break;
	case 0x7C : INC(ETEND(),7);break;
	case 0x0C : INC(DIREC(),6);break;
	case 0x6C : INC(INDEXE(),6);break;

// DEC
	case 0x4A : DECA();break;
	case 0x5A : DECB();break;
	case 0x7A : DEC(ETEND(),7);break;
	case 0x0A : DEC(DIREC(),6);break;
	case 0x6A : DEC(INDEXE(),6);break;

// BIT
	case 0x85 : BIT(A,IMMED8(),2);break;
	case 0x95 : BIT(A,DIREC(),4);break;
	case 0xB5 : BIT(A,ETEND(),5);break;
	case 0xA5 : BIT(A,INDEXE(),4);break;
	case 0xC5 : BIT(B,IMMED8(),2);break;
	case 0xD5 : BIT(B,DIREC(),4);break;
	case 0xF5 : BIT(B,ETEND(),5);break;
	case 0xE5 : BIT(B,INDEXE(),4);break;

// CMP
	case 0x81 : CMP8(A,IMMED8(),2);break;
	case 0x91 : CMP8(A,DIREC(),4);break;
	case 0xB1 : CMP8(A,ETEND(),5);break;
	case 0xA1 : CMP8(A,INDEXE(),4);break;
	case 0xC1 : CMP8(B,IMMED8(),2);break;
	case 0xD1 : CMP8(B,DIREC(),4);break;
	case 0xF1 : CMP8(B,ETEND(),5);break;
	case 0xE1 : CMP8(B,INDEXE(),4);break;
	case 0x8C : CMP16(X,IMMED16(),5);break;
	case 0x9C : CMP16(X,DIREC(),7);break;
	case 0xBC : CMP16(X,ETEND(),8);break;
	case 0xAC : CMP16(X,INDEXE(),7);break;

// TST
	case 0x4D : TSTAi();break;
	case 0x5D : TSTBi();break;
	case 0x0D : TST(DIREC(),6);break;
	case 0x7D : TST(ETEND(),7);break;
	case 0x6D : TST(INDEXE(),6);break;

// AND	
	case 0x84 : ANDA(IMMED8(),2);break;
	case 0x94 : ANDA(DIREC(),4);break;
	case 0xB4 : ANDA(ETEND(),5);break;
	case 0xA4 : ANDA(INDEXE(),4);break;
	case 0xC4 : ANDB(IMMED8(),2);break;
	case 0xD4 : ANDB(DIREC(),4);break;
	case 0xF4 : ANDB(ETEND(),5);break;
	case 0xE4 : ANDB(INDEXE(),4);break;
	case 0x1C : ANDCC(IMMED8(),3);break;

// OR	
	case 0x8A : ORA(IMMED8(),2);break;
	case 0x9A : ORA(DIREC(),4);break;
	case 0xBA : ORA(ETEND(),5);break;
	case 0xAA : ORA(INDEXE(),4);break;
	case 0xCA : ORB(IMMED8(),2);break;
	case 0xDA : ORB(DIREC(),4);break;
	case 0xFA : ORB(ETEND(),5);break;
	case 0xEA : ORB(INDEXE(),4);break;
	case 0x1A : ORCC(IMMED8(),3);break;

// EOR	
	case 0x88 : EORA(IMMED8(),2);break;
	case 0x98 : EORA(DIREC(),4);break;
	case 0xB8 : EORA(ETEND(),5);break;
	case 0xA8 : EORA(INDEXE(),4);break;
	case 0xC8 : EORB(IMMED8(),2);break;
	case 0xD8 : EORB(DIREC(),4);break;
	case 0xF8 : EORB(ETEND(),5);break;
	case 0xE8 : EORB(INDEXE(),4);break;

// COM
	case 0x43 : COMA();break;
	case 0x53 : COMB();break;
	case 0x03 : COM(DIREC(),6);break;
	case 0x73 : COM(ETEND(),7);break;
	case 0x63 : COM(INDEXE(),6);break;

// NEG
	case 0x40 : NEGA();break;
	case 0x50 : NEGB();break;
	case 0x00 : NEG(DIREC(),6);break;
	case 0x70 : NEG(ETEND(),7);break;
	case 0x60 : NEG(INDEXE(),6);break;

// ABX
	case 0x3A : ABX();break;

//ADD	
	case 0x8B : ADDA(IMMED8(),2);break;
	case 0x9B : ADDA(DIREC(),4);break;
	case 0xBB : ADDA(ETEND(),5);break;
	case 0xAB : ADDA(INDEXE(),4);break;

	case 0xCB : ADDB(IMMED8(),2);break;
	case 0xDB : ADDB(DIREC(),4);break;
	case 0xFB : ADDB(ETEND(),5);break;
	case 0xEB : ADDB(INDEXE(),4);break;

	case 0xC3 : ADDD(IMMED16(),4);break;
	case 0xD3 : ADDD(DIREC(),6);break;
	case 0xF3 : ADDD(ETEND(),7);break;
	case 0xE3 : ADDD(INDEXE(),6);break;

// ADC
	case 0x89 : ADCA(IMMED8(),2);break;
	case 0x99 : ADCA(DIREC(),4);break;
	case 0xB9 : ADCA(ETEND(),5);break;
	case 0xA9 : ADCA(INDEXE(),4);break;

	case 0xC9 : ADCB(IMMED8(),2);break;
	case 0xD9 : ADCB(DIREC(),4);break;
	case 0xF9 : ADCB(ETEND(),5);break;
	case 0xE9 : ADCB(INDEXE(),4);break;

// MUL
	case 0x3D : MUL();break;

// SBC
	case 0x82 : SBCA(IMMED8(),2);break;
	case 0x92 : SBCA(DIREC(),4);break;
	case 0xB2 : SBCA(ETEND(),5);break;
	case 0xA2 : SBCA(INDEXE(),4);break;

	case 0xC2 : SBCB(IMMED8(),2);break;
	case 0xD2 : SBCB(DIREC(),4);break;
	case 0xF2 : SBCB(ETEND(),5);break;
	case 0xE2 : SBCB(INDEXE(),4);break;

//SUB	
	case 0x80 : SUBA(IMMED8(),2);break;
	case 0x90 : SUBA(DIREC(),4);break;
	case 0xB0 : SUBA(ETEND(),5);break;
	case 0xA0 : SUBA(INDEXE(),4);break;

	case 0xC0 : SUBB(IMMED8(),2);break;
	case 0xD0 : SUBB(DIREC(),4);break;
	case 0xF0 : SUBB(ETEND(),5);break;
	case 0xE0 : SUBB(INDEXE(),4);break;

	case 0x83 : SUBD(IMMED16(),4);break;
	case 0x93 : SUBD(DIREC(),6);break;
	case 0xB3 : SUBD(ETEND(),7);break;
	case 0xA3 : SUBD(INDEXE(),6);break;

// SEX
	case 0x1D : SEX();break;

// ASL
	case 0x48 : ASLA();break;
	case 0x58 : ASLB();break;
	case 0x08 : ASL(DIREC(),6);break;
	case 0x78 : ASL(ETEND(),7);break;
	case 0x68 : ASL(INDEXE(),6);break;

// ASR
	case 0x47 : ASRA();break;
	case 0x57 : ASRB();break;
	case 0x07 : ASR(DIREC(),6);break;
	case 0x77 : ASR(ETEND(),7);break;
	case 0x67 : ASR(INDEXE(),6);break;

// LSR
	case 0x44 : LSRA();break;
	case 0x54 : LSRB();break;
	case 0x04 : LSR(DIREC(),6);break;
	case 0x74 : LSR(ETEND(),7);break;
	case 0x64 : LSR(INDEXE(),6);break;

// ROL
	case 0x49 : ROLA();break;
	case 0x59 : ROLB();break;
	case 0x09 : ROL(DIREC(),6);break;
	case 0x79 : ROL(ETEND(),7);break;
	case 0x69 : ROL(INDEXE(),6);break;

// ROR
	case 0x46 : RORA();break;
	case 0x56 : RORB();break;
	case 0x06 : ROR(DIREC(),6);break;
	case 0x76 : ROR(ETEND(),7);break;
	case 0x66 : ROR(INDEXE(),6);break;
	
// BRA 
  case 0x20 : BRA();break;
  case 0x16 : LBRA();break;

// JMP 
  case 0x0E : JMPd();break;
  case 0x7E : JMPe();break;
  case 0x6E : JMPx();break;

// BSR 
  case 0x8D : BSR();break;
  case 0x17 : LBSR();break;

// JSR 
  case 0x9D : JSRd();break;
  case 0xBD : JSRe();break;
  case 0xAD : JSRx();break;

	case 0x12 : NOP();break;
	case 0x39 : RTS();break;
  
// Bxx
	case 0x21 : BRN();break;
	case 0x24 : BCC();break;
	case 0x25 : BCS();break;
	case 0x27 : BEQ();break;
	case 0x26 : BNE();break;
	case 0x2C : BGE();break;
	case 0x2F : BLE();break;
	case 0x23 : BLS();break;
	case 0x2E : BGT();break;
	case 0x2D : BLT();break;
	case 0x22 : BHI();break;
	case 0x2B : BMI();break;
	case 0x2A : BPL();break;
	case 0x28 : BVC();break;
	case 0x29 : BVS();break;

	case 0x3F : SWI();break;
	case 0x3B  : RTI();break;
	case 0x19  : DAA();break;
	case 0x3C  : CWAI();break;
		
// extended mode
	case 0x10 :
	
	int opcode0x10 = mem.read(PC++);

	switch (opcode0x10) {

// LDS
	case 0xCE: S=LD16(IMMED16(),3);break;
	case 0xDE: S=LD16(DIREC(),5);break;
	case 0xFE: S=LD16(ETEND(),6);break;
	case 0xEE: S=LD16(INDEXE(),5);break;

// LDY
	case 0x8E : Y=LD16(IMMED16(),3);break;
	case 0x9E : Y=LD16(DIREC(),5);break;
	case 0xBE : Y=LD16(ETEND(),6);break;
	case 0xAE : Y=LD16(INDEXE(),5);break;

// STS
	case 0xDF : ST16(S,DIREC(),5);break;
	case 0xFF : ST16(S,ETEND(),6);break;
	case 0xEF : ST16(S,INDEXE(),5);break;

// STY
	case 0x9F : ST16(Y,DIREC(),5);break;
	case 0xBF : ST16(Y,ETEND(),6);break;
	case 0xAF : ST16(Y,INDEXE(),5);break;

// CMP
	case 0x83 : CALCD();CMP16(D,IMMED16(),5);break;
	case 0x93 : CALCD();CMP16(D,DIREC(),7);break;
	case 0xB3 : CALCD();CMP16(D,ETEND(),8);break;
	case 0xA3 : CALCD();CMP16(D,INDEXE(),7);break;
	case 0x8C : CMP16(Y,IMMED16(),5);break;
	case 0x9C : CMP16(Y,DIREC(),7);break;
	case 0xBC : CMP16(Y,ETEND(),8);break;
	case 0xAC : CMP16(Y,INDEXE(),7);break;

// Bxx
	case 0x21 : LBRN();break;
	case 0x24 : LBCC();break;
	case 0x25 : LBCS();break;
	case 0x27 : LBEQ();break;
	case 0x26 : LBNE();break;
	case 0x2C : LBGE();break;
	case 0x2F : LBLE();break;
	case 0x23 : LBLS();break;
	case 0x2E : LBGT();break;
	case 0x2D : LBLT();break;
	case 0x22 : LBHI();break;
	case 0x2B : LBMI();break;
	case 0x2A : LBPL();break;
	case 0x28 : LBVC();break;
	case 0x29 : LBVS();break;

	default : System.err.println("opcode 10 "+hex(opcode0x10,2)+" not implemented");
			  System.err.println(printState());
	} // of case opcode0x10
	break;
	case 0x11 :
	
	int opcode0x11 = mem.read(PC++);

	switch (opcode0x11) {

	// CMP
	case 0x8C : CMP16(S,IMMED16(),5);break;
	case 0x9C : CMP16(S,DIREC(),7);break;
	case 0xBC : CMP16(S,ETEND(),8);break;
	case 0xAC : CMP16(S,INDEXE(),7);break;
	case 0x83 : CMP16(U,IMMED16(),5);break;
	case 0x93 : CMP16(U,DIREC(),7);break;
	case 0xB3 : CMP16(U,ETEND(),8);break;
	case 0xA3 : CMP16(U,INDEXE(),7);break;

	default : System.err.println("opcode 11"+hex(opcode0x11,2)+" not implemented");
			  System.err.println(printState());
	} // of case opcode 0x11 
	break;
	
	default : System.err.println("opcode "+hex(opcode,2)+" not implemented");
			  System.err.println(printState());
	
	} // of case  opcode
} // of method fetch()


// UNASSEMBLE/DEBUG PART
	public String printState() {
		CC=getcc();
	String s="A="+hex(A,2)+" B="+hex(B,2);
	s+=" X="+hex(X,4)+" Y="+hex(Y,4)+"\n";
	s+="PC="+hex(PC,4)+" DP="+hex(DP,2);
	s+=" U="+hex(U,4)+" S="+hex(S,4);
	s+=" CC="+hex(CC,2);
	return s;
}

private String hex(int val,int size) {
	String output="";
	int t;
	int q;
	int mask;
	int coef;
	for (t=0;t<size;t++) {
		coef=(size-t-1)*4;
		mask=0xF << coef;
		q=((val&mask) >> coef);
		if (q<10) output=output+q;
		else
			{
					switch (q) {
						case 10 : output=output+"A";break;
						case 11 : output=output+"B";break;
						case 12 : output=output+"C";break;
						case 13 : output=output+"D";break;
						case 14 : output=output+"E";break;
						case 15 : output=output+"F";break;
						}
			}	
	}
	return output;
}

// force sign extension in a portable but ugly maneer
private int signedChar(int v) {
	if ((v&0x80)==0) return (v & 0xFF);
	int delta=-1; // delta is 0xFFFF.... independently of 32/64bits
  	delta=(delta>>8)<<8; // force last 8bits to 0
  	return (v&0xFF) | delta; // result is now signed
}

// force sign extension in a portable but ugly maneer
private int signed16bits(int v) {
	if ((v&0x8000)==0) return (v & 0xFFFF);
	int delta=-1; // delta is 0xFFFF.... independently of 32/64bits
  	delta=(delta>>16)<<16; // force last 16bits to 0
  	return (v&0xFFFF) | delta; // result is now signed
}

private String regx(int m)
{
	String output="?";
	if ((m&0x60)==0x00) output="X";
	if ((m&0x60)==0x20) output="Y";
	if ((m&0x60)==0x40) output="U";
	if ((m&0x60)==0x60) output="S";
	return output;
}

private String r_tfr(int m)
{
	String output="";
	switch (m&0xF0)
	{
	case 0x80 :   	output+="A,";
			break;
	case 0x90 :   	output+="B,";
			break;
	case 0xA0 :   	output+="CC,";
			break;
	case 0x00 :   	output+="D,";
			break;
	case 0xB0 :   	output+="DP,";
			break;
	case 0x50 :   	output+="PC,";
			break;
	case 0x40 :   	output+="S,";
			break;
	case 0x30 :   	output+="U,";
			break;
	case 0x10 :   	output+="X,";
			break;
	case 0x20 :   	output+="Y,";
			break;
	}
	switch (m&0x0F)
	{
	case 0x8 :   	output+="A";
			break;
	case 0x9 :   	output+="B";
			break;
	case 0xA :   	output+="CC";
			break;
	case 0x0 :   	output+="D";
			break;
	case 0xB :   	output+="DP";
			break;
	case 0x5 :   	output+="PC";
			break;
	case 0x4 :   	output+="S";
			break;
	case 0x3 :   	output+="U";
			break;
	case 0x1 :   	output+="X";
			break;
	case 0x2 :   	output+="Y";
			break;
	}
	return output;
}

private String r_pile(int m)
{
	String output="";
	if ((m&0x80)!=0) output+="PC,";
	if ((m&0x40)!=0) output+="U/S,";
	if ((m&0x20)!=0) output+="Y,";
	if ((m&0x10)!=0) output+="X,";
	if ((m&0x08)!=0) output+="DP,";
	if ((m&0x04)!=0) output+="B,";
	if ((m&0x02)!=0) output+="A,";
	if ((m&0x01)!=0) output+="CC";
	return output;
}



public String unassemble(int start,int maxLines) {
	String [] MNEMO=new String[256];
	String [] MNEMO10=new String[256];
	String [] MNEMO11=new String[256];

	String output="";

	String output1="";
	String output2="";
	
	// init all Strings
  	int l;

  	for (l=0;l<256;l++)
  	{
   		MNEMO[l]="ILL -";
		MNEMO10[l]="ILL -";
  		MNEMO11[l]="ILL -";
  	}

  /* LDA opcode */
  MNEMO[0x86]="LDA i";
  MNEMO[0x96]="LDA d";
  MNEMO[0xB6]="LDA e";
  MNEMO[0xA6]="LDA x";

  /* LDB opcode */
  MNEMO[0xC6]="LDB i";
  MNEMO[0xD6]="LDB d";
  MNEMO[0xF6]="LDB e";
  MNEMO[0xE6]="LDB x";

  /* LDD opcode */
  MNEMO[0xCC]="LDD I";
  MNEMO[0xDC]="LDD d";
  MNEMO[0xFC]="LDD e";
  MNEMO[0xEC]="LDD x";

  /* LDU opcode */
  MNEMO[0xCE]="LDU I";
  MNEMO[0xDE]="LDU d";
  MNEMO[0xFE]="LDU e";
  MNEMO[0xEE]="LDU x";

  /* LDX opcode */
  MNEMO[0x8E]="LDX I";
  MNEMO[0x9E]="LDX d";
  MNEMO[0xBE]="LDX e";
  MNEMO[0xAE]="LDX x";

  /* LDS opcode */
  MNEMO10[0xCE]="LDS I";
  MNEMO10[0xDE]="LDS d";
  MNEMO10[0xFE]="LDS e";
  MNEMO10[0xEE]="LDS x";

  /* LDY opcode */
  MNEMO10[0x8E]="LDY I";
  MNEMO10[0x9E]="LDY d";
  MNEMO10[0xBE]="LDY e";
  MNEMO10[0xAE]="LDY x";

  /* STA opcode */
  MNEMO[0x97]="STA d";
  MNEMO[0xB7]="STA e";
  MNEMO[0xA7]="STA x";

  /* STB opcode */
  MNEMO[0xD7]="STB d";
  MNEMO[0xF7]="STB e";
  MNEMO[0xE7]="STB x";

  /* STD opcode */
  MNEMO[0xDD]="STD d";
  MNEMO[0xFD]="STD e";
  MNEMO[0xED]="STD x";

  /* STS opcode */
  MNEMO10[0xDF]="STS d";
  MNEMO10[0xFF]="STS e";
  MNEMO10[0xEF]="STS x";

  /* STU opcode */
  MNEMO[0xDF]="STU d";
  MNEMO[0xFF]="STU e";
  MNEMO[0xEF]="STU x";

  /* STX opcode */
  MNEMO[0x9F]="STX d";
  MNEMO[0xBF]="STX e";
  MNEMO[0xAF]="STX x";

  /* STY opcode */
  MNEMO10[0x9F]="STY d";
  MNEMO10[0xBF]="STY e";
  MNEMO10[0xAF]="STY x";

  /* LEA opcode */
  MNEMO[0x32]="LEASx";
  MNEMO[0x33]="LEAUx";
  MNEMO[0x30]="LEAXx";
  MNEMO[0x31]="LEAYx";

  /* CLR opcode */
  MNEMO[0x0F]="CLR d";
  MNEMO[0x7F]="CLR e";
  MNEMO[0x6F]="CLR x";
  MNEMO[0x4F]="CLRA-";
  MNEMO[0x5F]="CLRB-";

  /* EXG */
  MNEMO[0x1E]="EXG r";

  /* TFR */
  MNEMO[0x1F]="TFR r";

  /* PSH */
  MNEMO[0x34]="PSHSR";
  MNEMO[0x36]="PSHUR";

  /* PUL */
  MNEMO[0x35]="PULSR";
  MNEMO[0x37]="PULUR";

  /* INC */
  MNEMO[0x4C]="INCA-";
  MNEMO[0x5C]="INCB-";
  MNEMO[0x7C]="INC e";
  MNEMO[0x0C]="INC d";
  MNEMO[0x6C]="INC x";

  /* DEC */
  MNEMO[0x4A]="DECA-";
  MNEMO[0x5A]="DECB-";
  MNEMO[0x7A]="DEC e";
  MNEMO[0x0A]="DEC d";
  MNEMO[0x6A]="DEC x";

  /* BIT */
  MNEMO[0x85]="BITAi";
  MNEMO[0x95]="BITAd";
  MNEMO[0xB5]="BITAe";
  MNEMO[0xA5]="BITAx";
  MNEMO[0xC5]="BITBi";
  MNEMO[0xD5]="BITBd";
  MNEMO[0xF5]="BITBe";
  MNEMO[0xE5]="BITBx";

  /* CMP */
  MNEMO[0x81]="CMPAi";
  MNEMO[0x91]="CMPAd";
  MNEMO[0xB1]="CMPAe";
  MNEMO[0xA1]="CMPAx";
  MNEMO[0xC1]="CMPBi";
  MNEMO[0xD1]="CMPBd";
  MNEMO[0xF1]="CMPBe";
  MNEMO[0xE1]="CMPBx";
  MNEMO10[0x83]="CMPDI";
  MNEMO10[0x93]="CMPDd";
  MNEMO10[0xB3]="CMPDe";
  MNEMO10[0xA3]="CMPDx";
  MNEMO11[0x8C]="CMPSI";
  MNEMO11[0x9C]="CMPSd";
  MNEMO11[0xBC]="CMPSe";
  MNEMO11[0xAC]="CMPSx";
  MNEMO11[0x83]="CMPUI";
  MNEMO11[0x93]="CMPUd";
  MNEMO11[0xB3]="CMPUe";
  MNEMO11[0xA3]="CMPUx";
  MNEMO[0x8C]="CMPXI";
  MNEMO[0x9C]="CMPXd";
  MNEMO[0xBC]="CMPXe";
  MNEMO[0xAC]="CMPXx";
  MNEMO10[0x8C]="CMPYI";
  MNEMO10[0x9C]="CMPYd";
  MNEMO10[0xBC]="CMPYe";
  MNEMO10[0xAC]="CMPYx";

  /* TST */
  MNEMO[0x4D]="TSTA-";
  MNEMO[0x5D]="TSTB-";
  MNEMO[0x0D]="TST d";
  MNEMO[0x7D]="TST e";
  MNEMO[0x6D]="TST x";

  /* AND */
  MNEMO[0x84]="ANDAi";
  MNEMO[0x94]="ANDAd";
  MNEMO[0xB4]="ANDAe";
  MNEMO[0xA4]="ANDAx";
  MNEMO[0xC4]="ANDBi";
  MNEMO[0xD4]="ANDBd";
  MNEMO[0xF4]="ANDBe";
  MNEMO[0xE4]="ANDBx";
  MNEMO[0x1C]="& CCi";

  /* OR */
  MNEMO[0x8A]="ORA i";
  MNEMO[0x9A]="ORA d";
  MNEMO[0xBA]="ORA e";
  MNEMO[0xAA]="ORA x";
  MNEMO[0xCA]="ORB i";
  MNEMO[0xDA]="ORB d";
  MNEMO[0xFA]="ORB e";
  MNEMO[0xEA]="ORB x";
  MNEMO[0x1A]="ORCCi";

  /* EOR */
  MNEMO[0x88]="EORAi";
  MNEMO[0x98]="EORAd";
  MNEMO[0xB8]="EORAe";
  MNEMO[0xA8]="EORAx";
  MNEMO[0xC8]="EORBi";
  MNEMO[0xD8]="EORBd";
  MNEMO[0xF8]="EORBe";
  MNEMO[0xE8]="EORBx";

  /* COM */
  MNEMO[0x03]="COM d";
  MNEMO[0x73]="COM e";
  MNEMO[0x63]="COM x";
  MNEMO[0x43]="COMA-";
  MNEMO[0x53]="COMB-";

  /* NEG */
  MNEMO[0x00]="NEG d";
  MNEMO[0x70]="NEG e";
  MNEMO[0x60]="NEG x";
  MNEMO[0x40]="NEGA-";
  MNEMO[0x50]="NEGB-";

  /* ABX */
  MNEMO[0x3A]="ABX -";

  /* ADC */
  MNEMO[0x89]="ADCAi";
  MNEMO[0x99]="ADCAd";
  MNEMO[0xB9]="ADCAe";
  MNEMO[0xA9]="ADCAx";
  MNEMO[0xC9]="ADCBi";
  MNEMO[0xD9]="ADCBd";
  MNEMO[0xF9]="ADCBe";
  MNEMO[0xE9]="ADCBx";

  /* ADD */
  MNEMO[0x8B]="ADDAi";
  MNEMO[0x9B]="ADDAd";
  MNEMO[0xBB]="ADDAe";
  MNEMO[0xAB]="ADDAx";
  MNEMO[0xCB]="ADDBi";
  MNEMO[0xDB]="ADDBd";
  MNEMO[0xFB]="ADDBe";
  MNEMO[0xEB]="ADDBx";
  MNEMO[0xC3]="ADDDI";
  MNEMO[0xD3]="ADDDd";
  MNEMO[0xF3]="ADDDe";
  MNEMO[0xE3]="ADDDx";

  /* MUL */
  MNEMO[0x3D]="MUL -";


  /* SBC */
  MNEMO[0x82]="SBCAi";
  MNEMO[0x92]="SBCAd";
  MNEMO[0xB2]="SBCAe";
  MNEMO[0xA2]="SBCAx";
  MNEMO[0xC2]="SBCBi";
  MNEMO[0xD2]="SBCBd";
  MNEMO[0xF2]="SBCBe";
  MNEMO[0xE2]="SBCBx";

  /* SUB */
  MNEMO[0x80]="SUBAi";
  MNEMO[0x90]="SUBAd";
  MNEMO[0xB0]="SUBAe";
  MNEMO[0xA0]="SUBAx";
  MNEMO[0xC0]="SUBBi";
  MNEMO[0xD0]="SUBBd";
  MNEMO[0xF0]="SUBBe";
  MNEMO[0xE0]="SUBBx";
  MNEMO[0x83]="SUBDI";
  MNEMO[0x93]="SUBDd";
  MNEMO[0xB3]="SUBDe";
  MNEMO[0xA3]="SUBDx";

  /* SEX */
  MNEMO[0x1D]="SEX -";

  /* ASL */
  MNEMO[0x08]="ASL d";
  MNEMO[0x78]="ASL e";
  MNEMO[0x68]="ASL x";
  MNEMO[0x48]="ASLA-";
  MNEMO[0x58]="ASLB-";

  /* ASR */
  MNEMO[0x07]="ASR d";
  MNEMO[0x77]="ASR e";
  MNEMO[0x67]="ASR x";
  MNEMO[0x47]="ASRA-";
  MNEMO[0x57]="ASRB-";

  /* LSR */
  MNEMO[0x04]="LSR d";
  MNEMO[0x74]="LSR e";
  MNEMO[0x64]="LSR x";
  MNEMO[0x44]="LSRA-";
  MNEMO[0x54]="LSRB-";

  /* ROL */
  MNEMO[0x09]="ROL d";
  MNEMO[0x79]="ROL e";
  MNEMO[0x69]="ROL x";
  MNEMO[0x49]="ROLA-";
  MNEMO[0x59]="ROLB-";

  /* ROR */
  MNEMO[0x06]="ROR d";
  MNEMO[0x76]="ROR e";
  MNEMO[0x66]="ROR x";
  MNEMO[0x46]="RORA-";
  MNEMO[0x56]="RORB-";

  /* BRA */
  MNEMO[0x20]="BRA o";
  MNEMO[0x16]="LBRAO";

  /* JMP */
  MNEMO[0x0E]="JMP d";
  MNEMO[0x7E]="JMP e";
  MNEMO[0x6E]="JMP x";

  /* BSR */
  MNEMO[0x8D]="BSR o";
  MNEMO[0x17]="LBSRO";

  /* JSR */
  MNEMO[0x9D]="JSR d";
  MNEMO[0xBD]="JSR e";
  MNEMO[0xAD]="JSR x";

  /* BRN */
  MNEMO[0x21]="BRN o";
  MNEMO10[0x21]="LBRNO";

  /* NOP */
  MNEMO[0x12]="NOP -";

  /* RTS */
  MNEMO[0x39]="RTS -";

  /* BCC */
  MNEMO[0x24]="BCC o";
  MNEMO10[0x24]="LBCCO";

  /* BCS */
  MNEMO[0x25]="BCS o";
  MNEMO10[0x25]="LBCSO";

  /* BEQ */
  MNEMO[0x27]="BEQ o";
  MNEMO10[0x27]="LBEQO";

  /* BNE */
  MNEMO[0x26]="BNE o";
  MNEMO10[0x26]="LBNEO";

  /* BGE */
  MNEMO[0x2C]="BGE o";
  MNEMO10[0x2C]="LBGEO";

  /* BLE */
  MNEMO[0x2F]="BLE o";
  MNEMO10[0x2F]="LBLEO";

  /* BLS */
  MNEMO[0x23]="BLS o";
  MNEMO10[0x23]="LBLSO";

  /* BGT */
  MNEMO[0x2E]="BGT o";
  MNEMO10[0x2E]="LBGTO";

  /* BLT */
  MNEMO[0x2D]="BLT o";
  MNEMO10[0x2D]="LBLTO";

  /* BHI */
  MNEMO[0x22]="BHI o";
  MNEMO10[0x22]="LBHIO";

  /* BMI */
  MNEMO[0x2B]="BMI o";
  MNEMO10[0x2B]="LBMIO";

  /* BPL */
  MNEMO[0x2A]="BPL o";
  MNEMO10[0x2A]="LBPLO";

  /* BVC */
  MNEMO[0x28]="BVC o";
  MNEMO10[0x28]="LBVCO";

  /* BVS */
  MNEMO[0x29]="BVS o";
  MNEMO10[0x29]="LBVSO";

  /* SWI1&3 */
  MNEMO[0x3F]="SWI i";
  MNEMO11[0x3F]="SWI3-";

  /* RTI */
  MNEMO[0x3B]="RTI -";


	int where=start;

  	String mnemo;
  	int mm;
  	int line;
  	for (line=0;line<maxLines;line++)
  	{
  	mm=mem.read(where);where++;

  	output1=hex(where-1,4)+".";
  	output1=output1+hex(mm,2)+" ";
  	output2="";

  	if (mm==0x10)
  	{
    	mm=mem.read(where);where++;
    	mnemo=MNEMO10[mm];
  		output1=output1+hex(mm,2)+" ";
    	output2=output2+mnemo.substring(0,4)+" ";
  	}
  	else
  	if (mm==0x11)
  	{
    	mm=mem.read(where);where++;
    	mnemo=MNEMO11[mm];
  		output1=output1+hex(mm,2)+" ";
    	output2=output2+mnemo.substring(0,4)+" ";
  	}
  	else
  	{
    	mnemo=MNEMO[mm];
    	output2=output2+mnemo.substring(0,4)+" ";
  	}
	switch (mnemo.charAt(4))
  	{
  		case 'I' :  mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+"#x"+hex(mm,2);
					mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+hex(mm,2);
					break;
  		case 'i' :	mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+"#x"+hex(mm,2);
  					break;
  		case 'e' :  mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+"x"+hex(mm,2);
					mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+hex(mm,2);
					break;
  		case 'd' :  mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+"x"+hex(mm,2);
					break;
  		case 'o' :  mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2=output2+signedChar(mm)+" (=x"+hex((where+signedChar(mm))&0xFFFF,4)+")";

					break;
  		case 'O' :  mm=mem.read(where)<<8;where++;
					mm|=mem.read(where);where++;
					output1=output1+hex(mm,4)+" ";
					output2=output2+signed16bits(mm)+" (=x"+hex((where+signed16bits(mm))&0xFFFF,4)+")";
					
					break;
  		case 'x' :	int mmx;
  					mmx=mem.read(where);where++;
					output1=output1+hex(mmx,2)+" ";
					if ((mmx&0x80)==0)
					{
						if ((mmx&0x10)==0)
						{
							output2+=(mmx&0x0F)+",";
							output2+=regx(mmx);
						}
						else
						{
							output2+="-"+(mmx&0x0F)+",";
							output2+=regx(mmx);
						}		
					}
					else
					switch (mmx&0x1F)
					{
						case 0x04 :	output2+=",";
							output2+=regx(mmx);
							break;
						case 0x14 :	output2+="[,";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x08 :   	mm=mem.read(where);where++;
							output1=output1+hex(mm,2)+" ";
							output2+=signedChar(mm)+",";
							output2+=regx(mmx);
							break;
						case 0x18 :   	mm=mem.read(where);where++;
							output1=output1+hex(mm,2)+" ";
							output2+="["+signedChar(mm)+",";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x09 :   	mm=mem.read(where)<<8;where++;
							mm|=mem.read(where);where++;
							output1=output1+hex(mm,4)+" ";
							output2+=signed16bits(mm)+",";
							output2+=regx(mmx);
							break;
						case 0x19 :   	mm=mem.read(where)<<8;where++;
							mm|=mem.read(where);where++;
							output1=output1+hex(mm,4)+" ";
							output2+="["+signed16bits(mm)+",";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x06 :   	output2+="A,";
							output2+=regx(mmx);
							break;
						case 0x16 :   	output2+="[A,";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x05 :   	output2+="B,";
							output2+=regx(mmx);
							break;
						case 0x15 :   	output2+="[B,";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x0B :   	output2+="D,";
							output2+=regx(mmx);
							break;
						case 0x1B :   	output2+="[D,";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x00 :   	output2+=",";
							output2+=regx(mmx);
							output2+="+";
							break;
						case 0x01 :   	output2+=",";
							output2+=regx(mmx);
							output2+="++";
							break;
						case 0x11 :   	output2+="[,";
							output2+=regx(mmx);
							output2+="++]";
							break;
						case 0x02 :   	output2+=",-";
							output2+=regx(mmx);
							break;
						case 0x03 :   	output2+=",--";
							output2+=regx(mmx);
							break;
						case 0x13 :   	output2+="[,--";
							output2+=regx(mmx);
							output2+="]";
							break;
						case 0x0C :   	mm=mem.read(where);where++;
							output1=output1+hex(mm,2)+" ";
							output2+=signedChar(mm)+",PC";
							break;
						case 0x1C :   	mm=mem.read(where);where++;
							output1=output1+hex(mm,2)+" ";
							output2+="["+signedChar(mm)+",PC";
							output2+="]";
							break;
						case 0x0D :   	mm=mem.read(where)<<8;where++;
							mm|=mem.read(where);where++;
							output1=output1+hex(mm,4)+" ";
							output2+=signed16bits(mm)+",PC";
							break;
						case 0x1D :   	mm=mem.read(where)<<8;where++;
							mm|=mem.read(where);where++;
							output1=output1+hex(mm,4)+" ";
							output2+="["+signed16bits(mm)+",PC]";
							break;
						case 0x1F :   	mm=mem.read(where)<<8;where++;
							mm|=mem.read(where);where++;										output1=output1+hex(mm,2)+" ";
							output1=output1+hex(mm,4)+" ";
							output2+="[x"+hex(mm,4)+"]";
							break;
						default   :	output2+="Illegal !";
	}

					break;
  		case 'r' :	mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
  					output2+=r_tfr(mm);
					break;
  		case 'R' :	mm=mem.read(where);where++;
					output1=output1+hex(mm,2)+" ";
					output2+=r_pile(mm);
					break;
  	}

	int ll;
	int lll=output1.length();
	for (ll=0;ll<32-lll;ll++) output1+=" ";
	output+=output1+output2+"\n";
	} // of for ... maxLines
	return output;
}
} // of class M6809
