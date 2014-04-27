package gui;

import javax.swing.event.*;
import javax.swing.*;
import java.io.*;
import java.awt.*;
import java.awt.event.*;
import java.util.*;

/**
   A simple Web Browser with minimal functionality.
   @author Jose M. Vidal
*/
public class Browser {
	private JFrame f;
	private Gui gui;
	/** Set the page.
		@param jep the pane on which to display the url
		@param url the url to display */
 	protected static void setPage(JEditorPane jep, String url){
		try {
			jep.setPage(url);
		}
		catch (IOException e) {
			System.err.println(e);
			System.exit(-1);
		}

	}

	/** An inner class which listens for keypresses on the Back button. */
	class backButtonListener implements ActionListener {
		protected JEditorPane jep;
		protected JLabel label;
		protected JButton backButton;
		protected Vector history;
		public backButtonListener(JEditorPane jep, JButton backButton, Vector history, JLabel label){
			this.jep = jep;
			this.backButton = backButton;
			this.history = history;
			this.label = label;
		}

		/** The action is to show the last url in the history.
		 @param e the event*/
		public void actionPerformed(ActionEvent e){
			try{
				//the current page is the last, remove it
				String curl = (String)history.lastElement();
				history.removeElement(curl);
					
				curl = (String)history.lastElement();
				System.out.println("Back to " + curl);
				setPage(jep,curl);
				label.setText("<html><b>URL:</b> "+ curl);
				if (history.size() == 1)
					backButton.setEnabled(false);
			}
			catch (Exception ex){
				System.out.println("Exception " + ex);
			}
		}
	}

	/** An inner class that listens for hyperlinkEvent.*/
	class LinkFollower implements HyperlinkListener {
		protected JEditorPane jep;
		protected JLabel label;
		protected JButton backButton;
		protected Vector history;
		public LinkFollower(JEditorPane jep, JButton backButton, Vector history, JLabel label){
			this.jep = jep;
			this.backButton = backButton; 
			this.history = history;
			this.label = label;
		}
		/** The action is to show the page of the URL the user clicked on.
			@param evt the event. We only care when its type is ACTIVATED. */
		public void hyperlinkUpdate(HyperlinkEvent evt){
			if (evt.getEventType() == HyperlinkEvent.EventType.ACTIVATED){
				try {
					String currentURL = evt.getURL().toString();
					// si l'url est un fichier zip .k5 ou .k7 => on le charge...
			        int index = currentURL.lastIndexOf ('.');
        			if (index != -1) {
					String extension = currentURL.substring (index + 1);
					System.out.println("ext " + extension);					
					if ((extension.equals("k5")) || (extension.equals("k7")) )
					{
						gui.setK7FromUrl(currentURL);
						f.dispose();
					}
					
        			}
					history.add(currentURL);
					backButton.setEnabled(true);
					System.out.println("Going to " + currentURL);
					setPage(jep,currentURL);
					label.setText("<html><b>URL:</b> "+ currentURL);
				}
				catch (Exception e) {
					System.out.println("ERROR: Trouble fetching url");
				}
			}
		}

	}

	/** The contructor runs the browser. It displays the main frame with the
		fetched initialPage
		@param initialPage the first page to show */

		
 	public Browser(String initialPage,Gui gui){

		/** A vector of String containing the past urls */
		Vector history = new Vector();
		history.add(initialPage);
		this.gui=gui;
		
		// set up the editor pane
		JEditorPane jep = new JEditorPane();
		jep.setEditable(false);
		setPage(jep, initialPage);

		// set up the window
		JScrollPane scrollPane = new JScrollPane(jep);     
		f = new JFrame("Simple Web Browser");
		f.setDefaultCloseOperation(WindowConstants.DISPOSE_ON_CLOSE);

		//Exit the program when user closes window.
		f.addWindowListener(new WindowAdapter() {
				public void windowClosing(WindowEvent e){
					f.dispose();
				}
			});

		//Label where we show the url
		JLabel label = new JLabel("<html><b>URL:</b> "+ initialPage);

		
		JButton backButton = new JButton ("Back");
		backButton.setActionCommand("back");
		backButton.setToolTipText("Go to previous page");
		backButton.setEnabled(false);
		backButton.addActionListener(new backButtonListener(jep, backButton, history, label));

		JButton exitButton = new JButton ("Exit");
		exitButton.setActionCommand("exit");
		exitButton.setToolTipText("Quit this application");
		exitButton.addActionListener(new ActionListener() {
				public void actionPerformed(ActionEvent e) {
					f.dispose();
				}
			});

		//A toolbar to hold all our buttons
		JToolBar toolBar = new JToolBar();
		toolBar.add(backButton);
		toolBar.add(exitButton);


		jep.addHyperlinkListener(new LinkFollower(jep, backButton, history, label));

		//Set up the toolbar and scrollbar in the contentpane of the frame
		JPanel contentPane = (JPanel)f.getContentPane();
		contentPane.setLayout(new BorderLayout());
		contentPane.setPreferredSize(new Dimension(400, 100));
		contentPane.add(toolBar, BorderLayout.NORTH);
		contentPane.add(scrollPane, BorderLayout.CENTER);
		contentPane.add(label, BorderLayout.SOUTH);

		f.pack();
		f.setSize(640, 360);
		f.setVisible(true);


	}

	/** Create a Browser object. Use the command-line url if given */
/*	public static void main(String[] args) {
		String initialPage = new String("http://www.cse.sc.edu");

		if (args.length > 0) initialPage = args[0];

		Browser b = new Browser(initialPage);
	}
*/	
}
