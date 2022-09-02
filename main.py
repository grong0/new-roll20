import scrapy
from scrapy.crawler import CrawlerProcess
import logging
from kivy.logger import Logger
logging.root = Logger

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




class allSpellsSpider(scrapy.Spider):
    name = "allSpells"
    start_urls = ['http://dnd5e.wikidot.com/spells']
    spell_urls = []
    new_loop = False

    def start_requests(self):
        yield scrapy.Request('http://dnd5e.wikidot.com/spells')

    def parse(self, response):
        if self.new_loop == False:
            for products in response.css('tr'):
                if products.css('th::text').get() == "Spell Name":
                    continue
                else:
                    # yield {
                    #     'spellName': products.css('a::text').get(),
                    #     'link': 'http://dnd5e.wikidot.com' + str(products.css('a::attr(href)').get()),
                    #     'school': products.css('em::text').get(),
                    #     'castingTime': products.css('td::text')[0].get(),
                    #     'range': products.css('td::text')[1].get(),
                    #     'duration': products.css('td::text')[2].get(),
                    #     'components': products.css('td::text')[3].get()
                    # }
                    self.spell_urls.append('http://dnd5e.wikidot.com' + str(products.css('a::attr(href)').get()))
            self.new_loop = True
        elif self.new_loop == True:
            content = response.css('div.main-content')
            source = content.css('p::text').get()[8:]
            castingTime = content.css('p::text').getall()[1][1:]
            spell_range = content.css('p::text').getall()[3][1:]
            components = content.css('p::text').getall()[5][1:]
            duration = content.css('p::text').getall()[7][1:]

            desc_up = content.css('p::text').getall()[8:len(content.css('p::text').getall()) - len(content.css('p').css('a::text').getall())]
            description = None
            upcast = None
            if desc_up[-1][0] == " ":
                description = desc_up[:len(desc_up) - 1]
                upcast = desc_up[-1][1:]
            else:
                description = desc_up
                upcast = ""

            desc_list = []
            for x in content.css('ul'):
                desc_list.append(x.css('em::text').get())
                if desc_list[len(desc_list) - 1] != None:
                    desc_list.append(x.css('li::text').get()[1:])
                else:
                    desc_list.append(x.css('li::text').get())

            spellLists = content.css('p').css('a::text').getall()

            # no_upcast = True
            # count = 1
            # paragraph = 8
            # while no_upcast:
            #     if not content.css('p::text').getall()[paragraph + count][1].isupper():
            #         description += "\n" + content.css('p::text').getall()[paragraph + count]
            #         count += 1
            #     else:
            #         no_upcast = False
            #         continue

            if "Critical Role" in content.css('p::text').get():
                source = "Critical Role"
                castingTime = content.css('p::text').getall()[2][1:]
                spell_range = content.css('p::text').getall()[4][1:]
                components = content.css('p::text').getall()[6][1:]
                duration = content.css('p::text').getall()[8][1:]

                desc_up = content.css('p::text').getall()[9:len(content.css('p::text').getall()) - len(content.css('p').css('a::text').getall())]
                description = None
                upcast = None
                if desc_up[-1][0] == " ":
                    description = desc_up[:len(desc_up) - 1]
                    upcast = desc_up[-1][1:]
                else:
                    description = desc_up
                    upcast = ""
                
                spellLists = content.css('p').css('a::text').getall()
                spellLists.remove("Twitter")
            yield {
                'spellName': response.css('div.page-title.page-header').css('span::text').get(),
                'source': source,
                'school+level': content.css('em::text').get(),
                'castingTime': castingTime,
                'range': spell_range,
                'components': components,
                'duration': duration,
                'description': description, #list
                'list': desc_list, #list
                'upcast': upcast,
                'spellLists': spellLists #list
            }
            source = ""
        for x in self.spell_urls:
            yield(scrapy.Request(x, callback=self.parse))

process = CrawlerProcess(settings={
    "FEEDS": {
        "spells.json": {
            "format": "json",
            "overwrite": True
        }
    }
})

process.crawl(allSpellsSpider)
process.start() # the script will block here until all crawling jobs are finished






TheLabApp().run()