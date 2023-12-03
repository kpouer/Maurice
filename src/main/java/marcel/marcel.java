package marcel;

import marcel.gui.Gui;

import java.applet.*;

public class marcel extends Applet{
    String open;
    String boot;
    public static void main(String[] args) {
        new Gui();
    }
    
    public void init() {
        open = this.getParameter("FILE");
        boot = this.getParameter("BOOT");
        Gui applet = new Gui(this.getCodeBase(), true);
        if (open!=null)
            applet.setK7FromUrl(this.getCodeBase() + open);
        applet.machine.setBoot(boot);
    }
}
