import string
import scrapy
from scrapy.crawler import CrawlerProcess

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

def isAlph(next, prev):
    alph = string.ascii_uppercase
    for x in alph:
        if prev == x:
            return False
        elif next == x:
            return True
        else:
            continue

def flipOrder(listIn):
    temp = []
    for x in range(0, len(listIn) - 1):
        temp.append(listIn[-x])
    return temp

class allRacesSpider(scrapy.Spider):
    name = "allSpells"
    start_urls = ['http://dnd5e.wikidot.com/']
    race_urls = []
    new_loop = False
    race_list = []


    def start_requests(self):
        yield scrapy.Request('http://dnd5e.wikidot.com/')

    def parse(self, response):
        # html = ""
        # race_class = ""
        # race = ""
        # race_link = "http://dnd5e.wikidot.com"
        # races = []
        # spanDetected = False
        # removingEndSpan = False
        # ahrefDetected = False
        # waitingForName = False
        # collectingName = False
        # collectingRaceLink = False
        # removingEndahref = False
        if self.new_loop == False:
            race_cat = response.css('div.feature.offcolor')[1].css('div.row')[1].css('div.col-sm-4').css('span::text').getall()[:response.css('div.feature.offcolor')[1].css('div.row')[1].css('div.col-sm-4').css('span::text').getall().index("Setting Specific")+1]
            spec_set = response.css('div.feature.offcolor')[1].css('div.row')[1].css('div.col-sm-4').css('span::text').getall()[response.css('div.feature.offcolor')[1].css('div.row')[1].css('div.col-sm-4').css('span::text').getall().index("Setting Specific")+1:]
            races = response.css('div.feature.offcolor')[1].css('div.row')[1].css('div.col-sm-4').css('a::text').getall()
            race_links = response.css('div.feature.offcolor')[1].css('div.row')[1].css('div.col-sm-4').css('a::attr(href)').getall()
            race_cat_count = 0
            spec_set_count = 0
            for x in range(0, len(races)):
                if race_cat[race_cat_count] == "Setting Specific":
                    if isAlph(races[x][0], self.race_list[-1]["race_name"][0]) == True:
                        spec_set_count += 1
                    self.race_list.append({
                        "race_name": races[x],
                        "race_cat": ["Setting Specific", spec_set[spec_set_count]],
                        "race_link": "http://dnd5e.wikidot.com" + race_links[x]
                    })
                    self.race_urls.append("http://dnd5e.wikidot.com" + race_links[x])
                else:
                    if len(self.race_list) >= 2 and races[x] != "Custom Lineage" and races[x] != "Fairy" and isAlph(races[x][0], self.race_list[-1]["race_name"][0]) == True:
                        race_cat_count += 1
                        if race_cat[race_cat_count] == "Setting Specific":
                            self.race_list.append({
                                "race_name": races[x],
                                "race_cat": ["Setting Specific", spec_set[spec_set_count]],
                                "race_link": "http://dnd5e.wikidot.com" + race_links[x]
                            })
                            self.race_urls.append("http://dnd5e.wikidot.com" + race_links[x])
                            continue
                    if races[x] == "Custom Lineage":
                        self.race_list.append({
                            "race_name": races[x],
                            "race_cat": "Custom Lineage",
                            "race_link": "http://dnd5e.wikidot.com" + race_links[x]
                        })
                        self.race_urls.append("http://dnd5e.wikidot.com" + race_links[x])
                        continue
                    self.race_list.append({
                        "race_name": races[x],
                        "race_cat": race_cat[race_cat_count],
                        "race_link": "http://dnd5e.wikidot.com" + race_links[x]
                    })
                    self.race_urls.append("http://dnd5e.wikidot.com" + race_links[x])
            self.new_loop = True
            yield {"race_urls": self.race_urls}
        elif self.new_loop == True: 
            feats = []
            race_name = response.css('div.page-title.page-header').css('span::text').get()
            try:
                for x in response.css('div.feature')[0].css('div.row')[0].css('ul'):
                    feats.append({
                        "feat_name": x.css('strong::text').get(),
                        "feat_desc": x.css('li::text').get()[1:]
                    })
            except:
                for x in response.css('div#page-content').css('ul'):
                    feats.append({
                        "feat_name": x.css('strong::text').get(),
                        "feat_desc": x.css('li::text').get()[1:]
                    })
            race_cat = None
            race_link = None
            for x in self.race_list:
                if "(UA)" in race_name:
                    if race_name[:4].lower() in x['race_name'].lower() and x['race_cat'] == "Unearthed Arcana":
                        race_cat = x['race_cat']
                        race_link = x['race_link']
                else:
                    if race_name[:4].lower() in x['race_name'].lower() and x['race_cat'] != "Unearthed Arcana":
                        race_cat = x['race_cat']
                        race_link = x['race_link']
            
            race_desc = []
            for x in response.css('p'):
                if type(x.css('em::text').get()) == str and x.css('em::text').get().__contains__('Mark of') == False:
                    race_desc.append(x.css('em::text').getall())
                elif type(x.css('strong::text').get()) == str and x.css('strong::text').get().__contains__('Mark of') == False:
                    race_desc.append(x.css('strong::text').getall())

            race_misc_table = {'misc_table_name': None}
            race_misc_table_rows = []
            pos = 1
            x = response.css('table.wiki-content-table')[0]
            if len(response.css('div.row')) > 1:
                x = response.css('div.row')[1]
            if type(x.css('th::text').get()) == str:
                if len(x.css('tr')[0].css('th').getall()) == 1:
                    race_misc_table['misc_table_name'] = x.css('th::text').get()
                if x.css('tr')[0].__contains__("td")
                if len(x.css('tr')[1].css('td').getall()) == 2:
                    race_misc_table_rows.append({
                        'cat1': x.css('tr')[1].css('th::text').get(),
                        'cat2': x.css('tr')[1].css('th::text')[1].get()
                    })
                    for y in x.css('tr')[2:]:
                        race_misc_table_rows.append({
                            "cat1_cont": y.css('td::text').get(),
                            "cat2_cont": y.css('td::text')[1].get()
                        })
                elif len(x.css('tr')[1].css('td').getall()) == 3:
                    race_misc_table_rows.append({
                        'cat1': x.css('th::text').get(),
                        'cat2': x.css('th::text')[1].get(),
                        'cat3': x.css('th::text')[2].get()
                    })
                    for y in x.css('tr')[1:]:
                        race_misc_table_rows.append({
                            "cat1_cont": y.css('td::text').get(),
                            "cat2_cont": y.css('td::text')[1].get(),
                            "cat3_cont": y.css('td::text')[2].get()
                        })
                race_misc_table['content'] = race_misc_table_rows
                race_misc_table_rows = []
            

            subraces = []
            subrace_feats = []
            subrace_desc_sublist = []
            sublist_names = []
            sublist_descs = []
            subrace_spell_table = {}
            subrace_spell_table_rows = []
            subrace_misc_table = {'misc_table_name': None}
            subrace_misc_table_rows = []
            for x in response.css('div.row')[2:]:
                for y in x.css('ul'):
                    if type(y.css('li').css('ul').css('strong::text').get()) == str:
                        sublist_names = y.css('li').css('ul').css('strong::text').getall()
                        sublist_descs = y.css('li').css('ul').css('li::text').getall()
                        for count, i in enumerate(sublist_names):
                            subrace_desc_sublist.append({
                                "sublist_name": sublist_names[count],
                                "sublist_desc": sublist_descs[count]
                            })
                        subrace_feats.append({
                            "subrace_feat_name": y.css('strong::text').get(),
                            "subrace_feat_desc": [y.css('li::text').get(), subrace_desc_sublist]
                        })
                        sublist_names = []
                        sublist_descs = []
                        subrace_desc_sublist = []
                    else:
                        try:
                            if y.css('strong::text').get() != subrace_feats[-1]["subrace_feat_desc"][1][0]["sublist_name"]:
                                continue
                        except:
                            subrace_feats.append({
                                "subrace_feat_name": y.css('strong::text').get(),
                                "subrace_feat_desc": y.css('li::text').get()
                            })
                if len(x.css('p')) > 1:
                    subrace_spell_table['spell_table_name'] = x.css('p')[-1].css('strong::text').get()
                    for y in x.css('table.wiki-content-table').css('tr')[1:]:
                        subrace_spell_table_rows.append({
                            "spell_level": y.css('td::text').get(),
                            "spell": y.css('a::text').getall()
                        })
                    subrace_spell_table['spell_table_content'] = subrace_spell_table_rows
                    subrace_spell_table_rows = []
                    subraces.append({
                        "subrace_name": x.css('span::text').get(),
                        "subrace_source": x.css('p::text').get()[8:],
                        "subrace_feats": subrace_feats,
                        "subrace_spell_table": subrace_spell_table
                    })
                elif type(x.css('th::text').get()) == str:
                    if len(x.css('tr')[0].css('th').getall()) == 1:
                        subrace_misc_table['misc_table_name'] = x.css('th::text').get()
                    if len(x.css('tr')[1].css('th').getall()) == 2:
                        subrace_misc_table_rows.append({
                            'cat1': x.css('tr')[1].css('th::text').get(),
                            'cat2': x.css('tr')[1].css('th::text')[1].get()
                        })
                        for y in x.css('tr')[2:]:
                            subrace_misc_table_rows.append({
                                "cat1_cont": y.css('td::text').get(),
                                "cat2_cont": y.css('td::text')[1].get()
                            })
                    elif len(x.css('tr')[1].css('td').getall()) == 3:
                        subrace_misc_table_rows.append({
                            'cat1': x.css('th::text').get(),
                            'cat2': x.css('th::text')[1].get(),
                            'cat3': x.css('th::text')[2].get()
                        })
                        for y in x.css('tr')[1:]:
                            subrace_misc_table_rows.append({
                                "cat1_cont": y.css('td::text').get(),
                                "cat2_cont": y.css('td::text')[1].get(),
                                "cat3_cont": y.css('td::text')[2].get()
                            })
                    subrace_misc_table['content'] = subrace_misc_table_rows
                    subrace_misc_table_rows = []
                    subraces.append({
                        "subrace_name": x.css('span::text').get(),
                        "subrace_source": x.css('p::text').get()[8:],
                        "subrace_feats": subrace_feats,
                        "subrace_misc_table": subrace_misc_table
                    })
                else:
                    subraces.append({
                        "subrace_name": x.css('span::text').get(),
                        "subrace_source": x.css('p::text').get()[8:],
                        "subrace_feats": subrace_feats
                    })
                subrace_feats = []
                subrace_spell_table = {}
                subrace_misc_table = {'misc_table_name': None}
            try:
                print(race_misc_table['content'])
                yield {
                    "race_name": race_name,
                    "race_cat": race_cat,
                    "race_link": race_link,
                    "race_desc": race_desc,
                    "race_feats": feats,
                    "race_misc_table": race_misc_table,
                    "race_subraces": subraces
                }
            except:
                yield {
                    "race_name": race_name,
                    "race_cat": race_cat,
                    "race_link": race_link,
                    "race_desc": race_desc,
                    "race_feats": feats,
                    "race_subraces": subraces
                }

        
        for x in flipOrder(self.race_urls):
            yield(scrapy.Request(x, callback=self.parse))


# allSpellsProcess = CrawlerProcess(settings={
#     "FEEDS": {
#         "spells.json": {
#             "format": "json",
#             "overwrite": True
#         }
#     }
# })

# allSpellsProcess.crawl(allSpellsSpider)
# allSpellsProcess.start() # the script will block here until all crawling jobs are finished

allRacesProcess = CrawlerProcess(settings={
    "FEEDS": {
        "races.json": {
            "format": "json",
            "overwrite": True
        }
    }
})

allRacesProcess.crawl(allRacesSpider)
allRacesProcess.start()





TheLabApp().run()