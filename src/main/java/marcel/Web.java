package marcel;

/**
 *
 * @author Markus
 */

import java.applet.Applet;
import marcel.gui.Gui;
import marcel.Selector;

import java.awt.BorderLayout;
import java.awt.Color;
import java.awt.event.*;
public class Web  extends Applet implements ActionListener{

    public void actionPerformed(ActionEvent e){
        if (e.getSource()==selector.bReset){
            applet.machine.resetHard();
            applet.machine.setK7FileFromUrl(this.getCodeBase() + open);
            applet.machine.setBoot(boot);
        }
        if (e.getSource()==selector.bFilter){
            applet.screen.filter = !applet.screen.filter;
            if (applet.screen.filter){
                selector.bFilter.setBackground(Color.GREEN);
            } else
                selector.bFilter.setBackground(Color.RED);
        }
    }
    Gui applet;
    Selector selector;
    String open;
    String boot;
    public void init() {
        selector = new Selector();
        open = this.getParameter("FILE");
        boot = this.getParameter("BOOT");
        applet = new Gui(this.getCodeBase(), false);
        if (open!=null)
            applet.setK7FromUrl(this.getCodeBase() + open);
        applet.machine.testtimer = 0;
        applet.machine.setBoot(boot);
        this.setLayout(new BorderLayout());
        this.add(applet.screen, BorderLayout.CENTER);
        this.add(selector, BorderLayout.SOUTH);
        selector.bFilter.addActionListener(this);
        selector.bReset.addActionListener(this);
    }

}
