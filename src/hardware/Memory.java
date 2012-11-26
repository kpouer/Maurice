package hardware;

import java.net.URL;
import java.io.*;


public class Memory {
	
// 0 1 			POINT 	2
// 2 3 			COLOR 	2
// 4 5 6 7   	RAM1 	4
// 8 9 10 11 	RAM2 	4
// 12			LINEA 	1
// 13 			LINEB 	1
// 14 15 16 17 	ROM 	4

    private int[][] mem;
	private int[] mapper;
	private boolean[] key;
	private boolean[] dirty;

/* Registres du 6821 */
	int ORA;
	int ORB;
	int DDRA;
	int DDRB;
	int CRA;
	public int CRB;

/* Registre du Gate Array */
	int GA0;
	int GA1;
	int GA2;
	public int GA3;

	private boolean appletMode=false;
    private URL appletCodeBase; 


	public Memory(URL appletCodeBase) {
    	this.appletMode=true;
		this.appletCodeBase=appletCodeBase;
		this.init();
	}
	
    public Memory() {
    	this.appletMode=false;
		this.init();
	}

    public void init() {
    	int i;
    	mem = new int[18][0x1000];
    	mapper = new int[16];
    	key=new boolean [256];
    	for (i=0;i<256;i++) key[i]=false;
    	
    	dirty=new boolean [200];
    	
      	mapper[0]=0;
    	mapper[1]=1;
    	mapper[2]=4;
    	mapper[3]=5;
    	mapper[4]=6;
    	mapper[5]=7;
    	mapper[6]=8;
    	mapper[7]=9;
    	mapper[8]=10;
    	mapper[9]=11;
    	mapper[10]=12;
    	mapper[11]=13;
    	mapper[12]=14;
    	mapper[13]=15;
    	mapper[14]=16;
    	mapper[15]=17;
    	
		reset();
    }
    // read with io
    public int read(int address) {
    	int page=(address&0xF000)>>12;
		return mem[mapper[page]][address&0xFFF];
    }
    
    // write with io
    public void write(int address, int value) {
    	int page=(address&0xF000)>>12;
    	if (address<0x1F40) dirty[address/40]=true;
    	if (page==0x0A) hardware(address,value);
    	else
    	mem[mapper[page]][address&0xFFF] = value&0xFF;
    }
    
    // read without io
    public int get(int address) {
    	int page=(address&0xF000)>>12;
		return mem[mapper[page]][address&0xFFF];
    }

    public void set(int address, int value) {
    	int page=(address&0xF000)>>12;
    	mem[mapper[page]][address&0xFFF] = value&0xFF;
    }
    
    public int POINT(int address) {
    	int page=(address&0xF000)>>12;
		return mem[page][address&0xFFF];
	}
	
	public int COLOR(int address) {
    	int page=(address&0xF000)>>12;
		return mem[page+2][address&0xFFF];
	}

	public boolean isDirty(int line) {
		boolean ret=dirty[line];
		dirty[line]=false;
		return ret;
	}
	
	public void setAllDirty() {
		int i;
		for (i=0;i<200;i++) dirty[i]=true;
	}
	
    public void reset() {
	int i;
	for (i = 0; i < 0xFFFF; i++) {
	    this.set(i,0x00);
	}
	loadRom();
		CRA=0x00;
	CRB=0x00;
	DDRA=0x5F;
	DDRB=0x7F;

	mem[0xA+2][0x7CC]=0xFF;
	mem[0xA+2][0x7CD]=0xFF;
	mem[0xA+2][0x7CE]=0xFF; 
	mem[0xA+2][0x7CF]=0xFF;
	
	patchK7();

    }
    public void loadRom() {

    	int startingAddress = 0xC000;

    	
    	if (!appletMode)
   		{
	    	String filename = System.getProperty("user.dir") + "/bios/mo5.rom";
	    	FileInputStream fis = null;
	    	try {
				fis = new FileInputStream(filename);
				int i;
				for (i = startingAddress; i < 0x10000; i++) {
		    		this.write(i,fis.read());
			}
			fis.close();
	    	}
	    	catch (Exception e) {
			System.out.println(e);
    		}
     	}
     	else {
     		DataInputStream fis = null;
	    	try{
				URL u = new URL(this.appletCodeBase,"bios/mo5.rom");
				fis = new DataInputStream (u.openStream());
				int i;
				for (i = startingAddress; i < 0x10000; i++) {
		    		this.write(i,fis.read());
				}
				fis.close();
	    	} catch (Exception e) {
				System.out.println(e);
				System.out.println("URL Error Access in Memory.class");
				return;
     		}
     }
}

private void hardware(int ADR,int OP) {
	/* 6821 système */
	/* acces à ORA ou DDRA */
	if (ADR==0xA7C0)
	{
		if ((CRA&0x04)==0x04)
		/* Accès à ORA */
		{
			if ((OP&0x01)==0x01)
			{
				mapper[0]=0;
				mapper[1]=1;
			}
			else
			{
				mapper[0]=2;
				mapper[1]=3;
			}
			/* Mise à jour de ORA selon le masque DDRA */
			ORA=(ORA&(DDRA^0xFF))|(OP&DDRA);
			mem[0xA+2][0x7C0]=ORA;
		}
		else
		{
			DDRA=OP;
			mem[0xA+2][0x7C0]=OP;
		}
	}
	else
	/* accès à ORB ou DDRB */
	if (ADR==0xA7C1)
	{
		if ((CRB&0x04)==0x04)
		/* Accès à ORB */
		{
         int O_ORB;

         O_ORB=ORB;

			ORB=(ORB&(DDRB^0xFF))|(OP&DDRB);

			/* GESTION HARD DU CLAVIER */

			if (key[ORB&0x7E])
			{
				ORB=ORB&0x7F;
			}
			else
			{
				ORB=ORB|0x80;
			}

			mem[0xA+2][0x7C1]=ORB;
		}
		else
		{
			DDRB=OP;
			mem[0xA+2][0x7C1]=OP;
		}
	}
	else
	/* accès à CRA */
	if (ADR==0xA7C2)
	{
		CRA=(CRA&0xD0)|(OP&0x3F);
		mem[0xA+2][0x7C2]=CRA;
	}
	else
	/* accès à CRB */
	if (ADR==0xA7C3)
	{
		CRB=(CRB&0xD0)|(OP&0x3F);
		mem[0xA+2][0x7C3]=CRB;
	}
  }
  public void setKey(int i) {
      key[i]=true;
  }
  	public void remKey(int i) {
        key[i]=false;
  	}
	
	int K7bit=0;
	int K7char=0;

    FileInputStream K7fis=null;
	boolean isFileOpened=false;
	DataInputStream K7in;	


    public boolean setK7FileFromUrl(String K7) {
        System.out.println("opening from url:"+ K7);
		try {
			URL site = new URL(K7);
			K7in =new DataInputStream (site.openStream());
			isFileOpened=true;
	    }
	    catch (Exception e) {
			System.out.println(e);
    	}

		K7bit=0;
		K7char=0;

    	return isFileOpened;
	}


    public boolean setK7File(String K7) {
        System.out.println("opening:"+ K7);
		try {
            if (K7fis==null)
                isFileOpened = false;
			if (isFileOpened) 
				 K7fis.close();
			K7fis=new FileInputStream(K7);
			 isFileOpened=true;
	    }
	    catch (Exception e) {
			System.out.println(e);
    	}

		K7bit=0;
		K7char=0;

    	return isFileOpened;
	}


	private	int readbit() {

		if (!isFileOpened) return 0;
	
		int octet;
	/* doit_on lire un caractere ? */
	if (K7bit==0x00)
	{
		try {
			K7char=K7in.read();
		}
	    catch (Exception e) {
            try{
			K7char=K7fis.read();}catch(Exception error){}
    	}

		K7bit=0x80;
	}
	octet=this.get(0x2045);

	if ((K7char&K7bit)==0)
	{
		octet=octet<<1;
		// A=0x00;
		this.set(0xF16A,0x00);
	}
	else
	{
		octet=(octet<<1)|0x01;
		// A=0xFF;
		this.set(0xF16A,0xFF);
	}
	/* positionne l'octet dans la page 0 du moniteur */
	this.set(0x2045,(octet&0xFF));
        Screen.led = octet & 0xff;
        Screen.show_led = 10;
	K7bit=K7bit>>1;
	return 0;
}


	public void periph(int PC,int S) {

		if (PC==0xF169) readbit();		
		
			/* Motor On/Off/Test */
		if (PC==0xF18C) {
			int c;
			/* Mise … 0 du bit C*/
			c=this.get(S);
			c&=0xFE;
			this.write((S),c);
		}

  	}
  
  	public void patchK7() {
		this.set(0xF18B,0x02);
		this.set(0xF18C,0x39);

		this.set(0xF168,0x02);

		// LDA immediate for return
		this.set(0xF169,0x86);
		this.set(0xF16A,0x00);
		
		this.set(0xF16B,0x39);
  	}
  
  	public void unPatchK7() {
  	}
}