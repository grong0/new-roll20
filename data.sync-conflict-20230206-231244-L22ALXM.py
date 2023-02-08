import string
import scrapy
from scrapy.crawler import CrawlerProcess
from scrapy.selector import Selector



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

def indexFromPoint(txt, char, index):
    for pos, x in enumerate(txt):
        if pos < index:
            continue
        if x == char:
            return pos

def indexInList(list, str):
    for index, x in enumerate(list):
        if str in x:
            return index

def elementsInDiv(code):

    code = code.extract()
    initialElementRemove = code[:code.index(">")+1]
    closingElementRemove = "</" + initialElementRemove[1:]
    code = code[len(initialElementRemove):-len(closingElementRemove)].replace("\n", "").replace('"', "'")
    numOfElements = code.count('/')
    if numOfElements == 0: return []
    hasEle = False
    currentElement = ""
    passingElement = ""
    pas = 0
    elements = []

    for ele in range(0, numOfElements):
        for index, x in enumerate(code):
            passingElement = code[index+1:indexFromPoint(code, ">", index)]
            if x == "<" and code[index+1] != "/":
                if currentElement == passingElement and hasEle == True:
                    pas += 1
                    continue
                if hasEle: continue
                currentElementIndex = index
                if " " in passingElement:
                    currentElement = code[index+1:indexFromPoint(code, " ", index)]
                else:
                    currentElement = passingElement
                hasEle = True
            elif x == "<" and passingElement == "/" + currentElement:
                if pas > 0:
                    pas -= 1
                    continue
                elements.append(code[currentElementIndex:indexFromPoint(code, ">", index)+1])
                code = code[:currentElementIndex] + code[indexFromPoint(code, ">", index)+1:]
                hasEle = False
                break
    return [x for x in elements if x != '']


def getRaceTableInfo(table):
    tab = []
    links = []

    for x in table.css('tr'):
        col = []
        for y in x.css('td'):
            text = x.css('td::text').get()
            link = None
            if "<a href" in y.extract():
                text = y.css('a::text').get()
                link = "http://dnd5e.wikidot.com" + y.css('a::attr(href)').get()
                links.append(link)
            col.append({
                "name": text,
                "link": link
            })
        tab.append(col)

    for index, row in enumerate(tab):
        if row == []:
            tab.pop(index)

    return tab, links


def toList(table):
    tableList = []
    for row in table:
        for col in row:
            if col['name'] != None:
                tableList.append(col)
    return tableList


def splitHeaders(page_content):
    if type(page_content) == scrapy.selector.unified.SelectorList:
        page_content = page_content[0]
    code = page_content.extract()
    headers = page_content.css("h1")

    contentList = []
    for index in range(len(headers)):
        currentHeaderCode = headers[index].extract()
        try:
            nextHeaderCode = headers[index+1].extract()
            content = code[code.index(currentHeaderCode):code.index(nextHeaderCode)]
        except:
            content = code[code.index(currentHeaderCode):-1]
            
        contentList.append(Selector(text=content))

    return contentList

def splitSubheaders(headers):
    if type(headers) == scrapy.selector.unified.SelectorList:
        headers = headers[0]
    code = headers.extract()
    subheaders = headers.css("h2")

    contentList = []
    for index in range(len(subheaders)):
        currentSubheaderCode = subheaders[index].extract()
        try:
            nextSubheaderCode = subheaders[index+1].extract()
            content = code[code.index(currentSubheaderCode):code.index(nextSubheaderCode)]
        except:
            content = code[code.index(currentSubheaderCode):-1]

        contentList.append(Selector(text=content))

    return contentList


def featsFromList(lists):
    feats = []
    for list in lists:
        name = list.css("li strong::text").get()
        desc = list.css("li::text").get()

        if name != None:
            name = name[:-1]
        if desc != None:
            desc = desc[1:]


        feats.append({
            "name": name,
            "desc": desc
        })
    return feats


def infoFromTable(table):
    tempTable = {
        "name": None,
        "header": [],
        "rows": []
    }

    for row in table.css("tr"):
        if row.css("th").get() != None:
            if len(row.css("th").getall()) == 1:
                tempTable['name'] = row.css("th::text").get()
            else:
                tempTable['header'].append(row.css("th::text").getall())
        else:
            tempTable['rows'].append(row.css("td::text").getall())

    return tempTable


def descFromParagraph(paragraph):
    if paragraph != []:
        allStrongs = paragraph.css("strong")
        allStrText = paragraph.css("strong::text").getall()
        allItalics = allStrongs[0].css("em").getall()
        allItaText = allStrongs[0].css("em::text").getall()

        combinedString = ""
        for index, x in enumerate(allStrText):
            combinedString += allItaText[index] + x
        if len(allStrText) != len(allItaText):
            combinedString += allItaText[-1]

        return combinedString


class allRacesSpider(scrapy.Spider):
    name = "allRaces"
    start_urls = ['http://dnd5e.wikidot.com/']
    race_urls = []
    new_loop = False
    lineages = []
    count = 0




    def start_requests(self):
        yield scrapy.Request('http://dnd5e.wikidot.com/lineage')

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

            page = response.css('div#page-content')
            
            for index, x in enumerate(page.css('h1')):
                name = x.css('span::text').get()
                table, links = getRaceTableInfo(page.css('table.wiki-content-table')[index])
                if name == "Unearthed Arcana":
                    table, links = getRaceTableInfo(page.css('table.wiki-content-table')[-1])
                    races = []
                    for row in table:
                        for col in row:
                            if col['link'] != None:
                                races.append(col)
                            else:
                                races[-1]['source'] = col['name']
                else:
                    races = toList(table)

                for link in links:
                    self.race_urls.append(link)
                
                if name == "Setting Specific Lineages":
                    self.lineages.append({
                        "name": name
                    })
                else:
                    self.lineages.append({
                        "name": name,
                        "races": races
                    })

            settingSpecificLineages = []
            for index, x in enumerate(page.css('h2')):
                name = x.css("span::text").get()
                table, links = getRaceTableInfo(page.css('table.wiki-content-table')[index + 4])
                for link in links:
                    self.race_urls.append(link)
                races = toList(table)
                settingSpecificLineages.append({
                    "name": name,
                    "races": races
                })
            self.lineages[-2]['lineages'] = settingSpecificLineages

            self.new_loop = True

            # yield {"race_urls": self.race_urls}




        elif self.new_loop == True:
            self.count += 1
            raceCount = 0
            for lineage in self.lineages: raceCount += len(lineage)
            if self.count == raceCount+1:
                print(f"lineages: {self.lineages}")
                for lineage in self.lineages:
                    yield lineage
                    pass

            raceFeats = []
            raceTable = None
            subraces = []
            raceDesc = []
            race_name = response.css('div.page-title.page-header').css('span::text').get()

            page_content = response.css("div#page-content")
            if page_content.css('h1').get() == None:
                feats = featsFromList(page_content.css("ul"))
                raceFeats.append(feats)
            elif "Dragonborn" in race_name:
                headers = splitHeaders(page_content)
                raceHeader = headers[0]
                raceFeats = featsFromList(raceHeader.css("ul"))
                
                if raceHeader.css("p") != None:
                    if type(raceHeader.css("p")) == Selector.selectorlist_cls:
                        for paragraph in raceHeader.css("p"): raceDesc.append(descFromParagraph(paragraph))
                    else:
                        raceDesc.append(descFromParagraph(raceHeader.css("p")))
                if raceHeader.css("table").get() != None:
                    raceTable = infoFromTable(raceHeader.css("table"))
                if len(headers) > 1:
                    otherSources = headers[1:]
                    
                    for otherSource in otherSources:
                        subraceSource = otherSource.css("h1 span::text").get()

                        subraceHeaders = splitSubheaders(otherSource)

                        for subraceHeader in subraceHeaders:

                            subraceName = subraceHeader.css("h2 span::text").get()
                            subraceFeats = featsFromList(subraceHeader.css("ul"))
                            subraceDesc = []
                            if subraceHeader.css("p") != None:
                                if type(subraceHeader.css("p")) == Selector.selectorlist_cls:
                                    for paragraph in subraceHeader.css("p"): subraceDesc.append(descFromParagraph(paragraph))
                                else:
                                    subraceDesc.append(descFromParagraph(subraceHeader.css("p")))
                            subraceTable = None
                            if subraceHeader.css("table").get() != None:
                                subraceTable = infoFromTable(subraceHeader.css("table"))

                            subraces.append({
                                "name": subraceName,
                                "source": subraceSource,
                                "desc": subraceDesc,
                                "feats": subraceFeats,
                                "table": subraceTable
                            })



            # for x in elementsInDiv(response.css('div.feature')[0].css('div.row')[0].css('div.col-lg-12').extract()[0]):
            #     if x[:x.index('>')+1] == '<ul>':
            #         race_feats.append({
            #             'feat_name': Selector(text=x).css('strong::text').get(),
            #             'feat_desc': Selector(text=x).css('li::text').get()
            #         })
            #     elif "table" in x[:x.index('>')+1]:
            #         race_feats[-1]['feat_table'] = {
            #             'table_header': Selector(text=x).css('th::text').getall(),
            #             'table_content': []
            #         }
            #         for row in Selector(text=x).css('tr')[2:]:
            #             race_feats[-1]['feat_table']['table_content'].append(row.css('td::text').getall())

        



            # for x in response.css('p'):
            #     if type(x.css('em::text').get()) == str and x.css('em::text').get().__contains__('Mark of') == False:
            #         race_desc.append(x.css('em::text').getall())
            #     elif type(x.css('strong::text').get()) == str and x.css('strong::text').get().__contains__('Mark of') == False:
            #         race_desc.append(x.css('strong::text').getall())





            # subrace_feats = []
            # for sub in response.css('div.row')[2:]:
            #     for index, x in enumerate(elementsInDiv(sub.css('div.col-lg-12').extract()[0])[1:]):
            #         if x[:x.index('>')+1] == '<ul>':
            #             content = []
            #             content.append({
            #                 'content': Selector(text=x).css('li::text').get(),
            #                 'inline': 0
            #             })
            #             subcontent = ""
            #             if "<ul>" in Selector(text=x).css('li').get():
            #                 if "<a href" in Selector(text=x).css('li').get()[:Selector(text=x).css('li').get().index("<ul>")]:
            #                     # index of a list element that has a sub_string in it:  index = [ind for ind, s in enumerate(sub_string) if '<ul>' in s][0]
            #                     stringList = Selector(text=x[:x.index('<ul>')] + "</li></ul>").css('li::text').getall()
            #                     for z in range(1, len(stringList)): stringList.insert(z+z-1, Selector(text=x).css('li a::text')[z-1].get())
            #                     for i in stringList: subcontent += i
            #                 for y in Selector(text=x).css('li ul li'):
            #                     if " " in y.get()[:y.get().index('>')+1]:
            #                         for z in y.css('li ul li'):
            #                             subcontent = z.css('li::text').get()
            #                             if "<a href" in y.css('li').get():
            #                                 subcontent = ''
            #                                 stringList = y.css('li::text').getall()
            #                                 for z in range(1, len(stringList)): stringList.insert(z+z-1, y.css('li a::text')[z-1].get())
            #                                 for i in stringList: subcontent += i
            #                             name = z.css('li strong::text').get()
            #                             content.append({
            #                                 'name': name,
            #                                 'content': subcontent,
            #                                 'inline': 2
            #                             })
            #                     else:
            #                         subcontent = y.css('li::text').get()
            #                         if "<a href" in y.css('li').get():
            #                             stringList = y.css('li::text').getall()
            #                             for z in range(1, len(stringList)): stringList.insert(z+z-1, y.css('li a::text')[z-1].get())
            #                             for i in stringList: subcontent += i
            #                         name = y.css('li strong::text').get()
            #                         content.append({
            #                             'name': name,
            #                             'content': subcontent,
            #                             'inline': 1
            #                         })
            #             elif "<a href" in Selector(text=x).css('li').get():
            #                 stringList = Selector(text=x).css('li::text').getall()
            #                 for z in range(1, len(stringList)): stringList.insert(z+z-1, Selector(text=x).css('li a::text')[z-1].get())
            #                 content[-1]['content'] = ''
            #                 for i in stringList: content[-1]['content'] += i
            #             subrace_feats.append({
            #                 'feat_name': Selector(text=x).css('strong::text').get(),
            #                 'feat_desc': content
            #             })
            #         elif 'table' in x[:x.index('>')+1]:
            #             name = None
            #             header = None
            #             pos = None
            #             if len(Selector(text=x).css('tr > th::text').getall()) > 1 and '<p>' in elementsInDiv(sub.css('div.col-lg-12').extract()[0])[index]:
            #                 name = Selector(text=elementsInDiv(sub.css('div.col-lg-12').extract()[0])[index]).css('p strong::text').get()
            #                 header = Selector(text=x).css('tr > th::text').getall()
            #                 pos = 1
            #             else:
            #                 name = Selector(text=x).css('tr > th::text').get()
            #                 header = Selector(text=x).css('tr')[1].css('th::text').getall()
            #                 pos = 2
            #             subrace_feats[-1]['feat_table'] = {
            #                 'table_name': name,
            #                 'table_header': header,
            #                 'table_content': []
            #             }
            #             for row in Selector(text=x).css('tr')[pos:]:
            #                 if '<a href' in row.get():
            #                     st = ""
            #                     tab = row.css('td::text').getall()
            #                     stringList = row.css('td::text').getall()
            #                     if ', ' in stringList:
            #                         stringList = [stringList[indexInList(row.css('td').getall(), '<a href')]]
            #                         for z in range(0, stringList.count(', ')+1): stringList.insert(z+z, row.css('td a::text').getall()[z])
            #                         for i in stringList: st += i
            #                         tab[indexInList(row.css('td').getall(), '<a href')] = st
            #                     elif len(stringList) == 1:
            #                         tab.append(row.css('td a::text').get())
            #                     else:
            #                         stringList = [stringList[indexInList(row.css('td').getall(), '<a href')]]
            #                         for z in range(1, len(stringList)): stringList.insert(z+z-1, row.css('td a::text').getall()[z-1])
            #                         for i in stringList: st += i
            #                         tab[indexInList(row.css('td').getall(), '<a href')] = st
            #                     subrace_feats[-1]['feat_table']['table_content'].append(tab)
            #                 else:
            #                     subrace_feats[-1]['feat_table']['table_content'].append(row.css('td::text').getall())
            #     subraces.append({
            #         'subrace_name': sub.css('h3 > span::text').get(),
            #         'subrace_source': sub.css('p::text').get()[sub.css('p::text').get().index(" ")+1:],
            #         'subrace_feats': subrace_feats
            #     })
            #     subrace_feats = []



            # You can optomise here by creating a dummy list that is equal to self.lineages and as 
            # you find races, you can get rid of that race in the list, making the future searches
            # faster

            

            race_name = response.css('div.page-title.page-header').css('span::text').get()
            for lineage in self.lineages:
                try:
                    for race in lineage['races']:
                        if "(UA)" in race_name:
                            if race_name[:4].lower() in race['name'].lower() and lineage['name'] == "Unearthed Arcana":
                                race['feats'] = raceFeats
                                race['desc'] = raceDesc
                                race['table'] = raceTable
                                race['subraces'] = subraces
                        else:
                            if race_name.lower() in race['name'].lower() and lineage['name'] != "Unearthed Arcana":
                                race['feats'] = raceFeats
                                race['desc'] = raceDesc
                                race['table'] = raceTable
                                race['subraces'] = subraces
                except:
                    for extraLineage in lineage['lineages']:
                        for race in extraLineage['races']:
                            race['feats'] = raceFeats
                            race['desc'] = raceDesc
                            race['table'] = raceTable
                            race['subraces'] = subraces



        
        for index, x in enumerate(flipOrder(self.race_urls)):
            yield(scrapy.Request(x, callback=self.parse))
    

class testSpider(scrapy.Spider):
    name = "test"
    start_urls = ['http://dnd5e.wikidot.com/dragonborn']

    def parse(self, response):
        subraces = []
        subrace_feats = []
        for sub in response.css('div.row')[2:]:
            for index, x in enumerate(elementsInDiv(sub.css('div.col-lg-12').extract()[0])[1:]):
                if x[:x.index('>')+1] == '<ul>':
                    content = []
                    content.append({
                        'content': Selector(text=x).css('li::text').get(),
                        'inline': 0
                    })
                    subcontent = ""
                    if "<ul>" in Selector(text=x).css('li').get():
                        if "<a href" in Selector(text=x).css('li').get()[:Selector(text=x).css('li').get().index("<ul>")]:
                            # index of a list element that has a sub_string in it:  index = [ind for ind, s in enumerate(sub_string) if '<ul>' in s][0]
                            stringList = Selector(text=x[:x.index('<ul>')] + "</li></ul>").css('li::text').getall()
                            for z in range(1, len(stringList)): stringList.insert(z+z-1, Selector(text=x).css('li a::text')[z-1].get())
                            for i in stringList: subcontent += i
                        if len(Selector(text=x).css('li ul li')) > 1:
                            print(f"Selector: {Selector(text=x).css('li ul li').getall()}")
                            print(f"elementsInDiv: {Selector(text=x).css('li ul').extract()[0]}")
                            pass
                        for y in Selector(text=x).css('li ul li'):
                            if " " in y.get()[:y.get().index('>')+1]:
                                for z in y.css('li ul li'):
                                    subcontent = z.css('li::text').get()
                                    if "<a href" in y.css('li').get():
                                        subcontent = ''
                                        stringList = y.css('li::text').getall()
                                        for z in range(1, len(stringList)): stringList.insert(z+z-1, y.css('li a::text')[z-1].get())
                                        for i in stringList: subcontent += i
                                    name = z.css('li strong::text').get()
                                    content.append({
                                        'name': name,
                                        'content': subcontent,
                                        'inline': 2
                                    })
                            else:
                                subcontent = y.css('li::text').get()
                                if "<a href" in y.css('li').get():
                                    stringList = y.css('li::text').getall()
                                    for z in range(1, len(stringList)): stringList.insert(z+z-1, y.css('li a::text')[z-1].get())
                                    for i in stringList: subcontent += i
                                name = y.css('li strong::text').get()
                                content.append({
                                    'name': name,
                                    'content': subcontent,
                                    'inline': 1
                                })
                    elif "<a href" in Selector(text=x).css('li').get():
                        stringList = Selector(text=x).css('li::text').getall()
                        for z in range(1, len(stringList)): stringList.insert(z+z-1, Selector(text=x).css('li a::text')[z-1].get())
                        content[-1]['content'] = ''
                        for i in stringList: content[-1]['content'] += i
                    subrace_feats.append({
                        'feat_name': Selector(text=x).css('strong::text').get(),
                        'feat_desc': content
                    })
                elif 'table' in x[:x.index('>')+1]:
                    name = None
                    header = None
                    pos = None
                    if len(Selector(text=x).css('tr > th::text').getall()) > 1 and '<p>' in elementsInDiv(sub.css('div.col-lg-12').extract()[0])[index]:
                        name = Selector(text=elementsInDiv(sub.css('div.col-lg-12').extract()[0])[index]).css('p strong::text').get()
                        header = Selector(text=x).css('tr > th::text').getall()
                        pos = 1
                    else:
                        name = Selector(text=x).css('tr > th::text').get()
                        header = Selector(text=x).css('tr')[1].css('th::text').getall()
                        pos = 2
                    subrace_feats[-1]['feat_table'] = {
                        'table_name': name,
                        'table_header': header,
                        'table_content': []
                    }
                    for row in Selector(text=x).css('tr')[pos:]:
                        if '<a href' in row.get():
                            st = ""
                            tab = row.css('td::text').getall()
                            stringList = row.css('td::text').getall()
                            if ', ' in stringList:
                                stringList = [stringList[indexInList(row.css('td').getall(), '<a href')]]
                                for z in range(0, stringList.count(', ')+1): stringList.insert(z+z, row.css('td a::text').getall()[z])
                                for i in stringList: st += i
                                tab[indexInList(row.css('td').getall(), '<a href')] = st
                            elif len(stringList) == 1:
                                tab.append(row.css('td a::text').get())
                            else:
                                stringList = [stringList[indexInList(row.css('td').getall(), '<a href')]]
                                for z in range(1, len(stringList)): stringList.insert(z+z-1, row.css('td a::text').getall()[z-1])
                                for i in stringList: st += i
                                tab[indexInList(row.css('td').getall(), '<a href')] = st
                            subrace_feats[-1]['feat_table']['table_content'].append(tab)
                        else:
                            subrace_feats[-1]['feat_table']['table_content'].append(row.css('td::text').getall())
            subraces.append({
                'subrace_name': sub.css('h3 > span::text').get(),
                'subrace_source': sub.css('p::text').get()[sub.css('p::text').get().index(" ")+1:],
                'subrace_feats': subrace_feats
            })
            subrace_feats = []
        print(subraces)
        pass



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

# testProcess = CrawlerProcess(settings={
#     "FEEDS": {
#         "test.json": {
#             "format": "json",
#             "overwrite": True
#         }
#     }
# })

# testProcess.crawl(testSpider)
# testProcess.start()