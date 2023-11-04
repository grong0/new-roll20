from kivy.app import App
from kivy.uix.gridlayout import GridLayout
from kivy.uix.label import Label
from kivy.uix.image import Image
from kivy.uix.button import Button
from kivy.uix.textinput import TextInput

class SayHello(App):
    def build(self):
        self.window = GridLayout()
        # add widgets to window
        
        # image widget
        self.window.add_widget(Image(source=""))

        return self.window

if __name__ == '__main__':
    SayHello().run()