from kivy.app import App
from kivy.metrics import dp
from kivy.properties import StringProperty, BooleanProperty
from kivy.uix.button import Button
from kivy.uix.widget import Widget
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.anchorlayout import AnchorLayout
from kivy.uix.gridlayout import GridLayout
from kivy.uix.stacklayout import StackLayout
from kivy.uix.scrollview import ScrollView
from kivy.uix.pagelayout import PageLayout

class WidgetsExample(GridLayout):
    my_text = StringProperty("1")
    state = BooleanProperty(False)
    text_input_str = StringProperty("foo")
    def on_button_click(self):
        if self.state:
            self.my_text = str(int(self.my_text) + 1)
            print(self.my_text)

    def on_toggle_button_state(self, widget):
        print("toggle state: " + widget.state)
        if widget.state == "normal":
            widget.text = "OFF"
            self.state = False
        elif widget.state == "down":
            widget.text = "ON"
            self.state = True

    def on_switch_active(self, widget):
        print("Switch: " + str(widget.active))

    def on_text_validate(self, widget):
        self.text_input_str = widget.text


# class CorePageRoll20(ScrollView):
#     pass

class PageLayoutExample(PageLayout):
    pass

class ScrollViewExample(ScrollView):
    pass

class StackLayoutExample(StackLayout):
    #pass
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        # left-right  top-bottom
        # self.orientation = "rl-bt"
        for i in range(0, 100):
            b = Button(text=str(i + 1), size_hint=(None, None), size=(dp(100), dp(100)))
            self.add_widget(b)

class GridLayoutExample(GridLayout):
    pass

class AnchorLayoutExample(AnchorLayout):
    pass

class BoxLayoutExample(BoxLayout):
    pass
    """
    def __init__(self, **kwargs):
        super().__init__(**kwargs)

        self.orientation = "vertical" #Defualt is horizontal

        b1 = Button(text="A")
        b2 = Button(text="B")

        self.add_widget(b1)
        self.add_widget(b2)
        """


class MainWidget(Widget):
    pass

class TheLabApp(App):
    pass 



TheLabApp().run()