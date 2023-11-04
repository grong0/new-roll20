import string
import scrapy
from scrapy.crawler import CrawlerProcess
from scrapy.selector import Selector
import webbrowser
import os

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
                    self.spell_urls.append(
                        'http://dnd5e.wikidot.com' + str(products.css('a::attr(href)').get()))
            self.new_loop = True
        elif self.new_loop == True:
            name = response.css('div.page-title.page-header').css('span::text').get()
            content = response.css('div.main-content')
            source = content.css('p::text').get()[8:]
            castingTime = content.css('p::text').getall()[1][1:]
            spell_range = content.css('p::text').getall()[3][1:]

            component_data = content.css('p::text').getall()[5][1:]
            
            duration = content.css('p::text').getall()[7][1:]

            schData = content.css('em::text').get().split(" ")
            school = schData[0] if "level" not in schData[0] else schData[1][0].upper() + schData[1][1:]
            level = schData[1] if "level" not in schData[0] else schData[0][:schData[0].find("-")]
            ritual = True if len(schData) == 3 else False

            desc_up = content.css('p::text').getall()[8:len(content.css(
                'p::text').getall()) - len(content.css('p').css('a::text').getall())]
            description = None
            upcast = None
            
            if "decompose" in name:
                print()

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
                component_data = content.css('p::text').getall()[6][1:]
                duration = content.css('p::text').getall()[8][1:]

                desc_up = content.css('p::text').getall()[9:len(content.css(
                    'p::text').getall()) - len(content.css('p').css('a::text').getall())]
                description = None
                upcast = None
                
                if "decompose" in name:
                    print()
                
                if desc_up[-1][0] == " ":
                    description = desc_up[:len(desc_up) - 1]
                    upcast = desc_up[-1][1:]
                else:
                    description = desc_up
                    upcast = ""

                spellLists = content.css('p').css('a::text').getall()
                spellLists.remove("Twitter")

            materials = component_data[component_data.find("("):component_data.find(")")][1:]
            components = component_data[:component_data.find(" (")]
            component_obj = {
                "V": True if "V" in components else False,
                "S": True if "S" in components else False,
                "M": True if materials != "" else False,
                "items": materials
            }
            
            yield {
                'spellName': name,
                'source': source,
                'school': school,
                'level': level,
                'ritual': ritual,
                'castingTime': castingTime,
                'range': spell_range,
                'components': component_obj,
                'duration': duration,
                'description': description,  # list
                'list': desc_list,  # list
                'upcast': upcast,
                'spellLists': spellLists  # list
            }
            source = ""
        for x in self.spell_urls:
            yield (scrapy.Request(x, callback=self.parse))


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
        if txt[pos:pos+len(char)] == char:
            return pos


def indexInList(list, str):
    for index, x in enumerate(list):
        if str in x:
            return index


def removeNothing(list: list) -> list:
    for item in list:
        if item == "":
            list.remove(item)
    return list



def elementsInDiv(code: str) -> list:
    code = code.extract()
    main_element = code[1:code.find(">")]
    if " " in main_element: main_element = main_element[:main_element.find(" ")]
    code = removeIndent(code)[code.find(">")+1:-len(main_element)-3]

    new_elements = []
    
    looking = False
    sub_children = 0
    target = {
        "ele": "",
        "index": 0,
    }
    
    for index, char in enumerate(code):
        if char == "<":
            element = code[index:code.find(">", index)+1]
            if "<br" in element: continue
            if "/" in element and looking:
                if sub_children == 0:
                    new_elements.append(code[target['index']:index+len(target['ele'])+3])
                    looking = False
                else: sub_children -= 1
            else:
                if looking: sub_children += 1
                else:
                    target = {
                        "ele": code[index+1:code.find(">", index)],
                        "index": index
                    }
                    looking = True
    return new_elements


def getRaceTableInfo(table) -> (list, list):
    tab = []
    links = []

    for x in table.css('tr'):
        col = []
        for y in x.css('td'):
            text = x.css('td::text').get()
            link = None
            if "<a href" in y.extract():
                text = y.css('a::text').get()
                relative_link = y.css('a::attr(href)').get()
                if "http://dnd5e.wikidot.com" in relative_link: link = relative_link
                else: link = "http://dnd5e.wikidot.com" + y.css('a::attr(href)').get()
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


def toList(table) -> list:
    tableList = []
    for row in table:
        for col in row:
            if col['name'] != None:
                tableList.append(col)
    return tableList


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

        source = None
        if paragraph.css("p::text").get() != None and "Source" in paragraph.css("p::text").get():
            source = paragraph.css("p::text").get()[8:]

        allStrongs = paragraph.css("strong")
        if allStrongs == []:
            return None, source

        allStrText = paragraph.css("strong::text").getall()
        allItaText = allStrongs[0].css("em::text").getall()

        if allStrText == []:
            return allItaText[0], source

        combinedString = ""
        for index, x in enumerate(allStrText):
            combinedString += (allItaText[index] + x)
        if len(allItaText) != len(allStrText):
            combinedString += allItaText[-1]
        return combinedString, source
    

def getElementName(element: str) -> str:
    """The name of the element

    Args:
        element (str): an element's code1

    Returns:
        str: the element's name
    """
    if "/" in element: inside = element[2:-1]
    else: inside = element[1:-1]
    return inside[:inside.find(" ")]


def getHeadersWithinDiv(div: Selector) -> list:
    """Header elements within an element

    Args:
        div (Selector): element to search in

    Returns:
        list: list of hearders
    """
    elementsInDiv(div.extract())
    headers = []
    for element in elementsInDiv:
        if len(getElementName(element)) and "h" in getElementName(element):
            headers.append(element)
    return headers


def getHeaders(page_content):
    
    pageContentCode = page_content.extract()[0].replace("\n", "")
    headers = page_content.css("h1")
    headerElement = "h1"
    if len(headers) == 0: 
        headers = page_content.css("h2")
        headerElement = "h2"
        if len(headers) == 0:
            return None, None
    headersCode = headers.extract()

    if len(headersCode) == 1:
        return page_content, headerElement

    headersCodeContent = []
    for index in range(0, len(headersCode)-1):
        currentHeaderIndex = pageContentCode.index(headersCode[index])
        nextHeaderIndex = pageContentCode.index(headersCode[index+1])
        currentHeaderCodeContent = pageContentCode[currentHeaderIndex:nextHeaderIndex]
        headersCodeContent.append(currentHeaderCodeContent)
    indexOfLastHeader = pageContentCode.index(headersCode[-1])
    indexOfDiv = indexFromPoint(pageContentCode, "</div>", indexOfLastHeader)
    headersCodeContent.append(pageContentCode[indexOfLastHeader:indexOfDiv])

    headersContent = []
    for headerCodeContent in headersCodeContent:
        headerContent = Selector(text=headerCodeContent)
        headersContent.append(headerContent)

    return headersContent, headerElement


def getSubHeaders(header, headerElement):
    headerContentCode = header.extract()[0]
    subHeaders = header.css(headerElement)
    subHeadersCode = subHeaders.extract()

    if len(subHeadersCode) == 1:
        return None

    subHeadersCodeContent = []
    for index in range(0, len(subHeadersCode)-1):
        currentHeaderIndex = headerContentCode.index(subHeadersCode[index])
        nextHeaderIndex = headerContentCode.index(subHeadersCode[index])
        currentHeaderCodeContent = headerContentCode[currentHeaderIndex:nextHeaderIndex]
        subHeadersCodeContent.append(currentHeaderCodeContent)

    subHeadersContent = []
    for subHeaderCodeContent in subHeadersContent:
        subHeaderContent = Selector(text=subHeaderCodeContent)
        subHeadersContent.append(subHeaderContent)

    return subHeadersContent


def getRaceCode(page_content, header, subheader, headerElement: str) -> Selector:
    
    headers = getHeadersWithinDiv(page_content)
    tiers = []
    for header in headers:
        if header[-1] not in tiers: tiers.append(header[-1])
    tiers.sort(reverse=True)
    
    headerCode = header.css("body").extract()[0]
    firstHeader = header.css(headerElement)
    var1 = header.extract()
    var2 = subheader.extract()
    firstHeaderCode = header.css(headerElement).getall()
    if type(firstHeaderCode) == list: firstHeaderCode = firstHeaderCode[0]
    indexOfHeader = headerCode.index(firstHeaderCode)
    subheaderCode = subheader.extract()
    indexOfSubHeader = headerCode.index(subheaderCode)
    idexOfEndOfHeader = indexFromPoint(headerCode, ">", indexOfHeader)

    mainRaceCode = headerCode[idexOfEndOfHeader+1:indexOfSubHeader]
    mainRaceElement = Selector(text=mainRaceCode)

    return mainRaceElement


def getRaceName(response) -> str:
    pageHeader = response.css("div.page-title.page-header")
    spanText = pageHeader.css("span::text").get()

    processedSpanText = spanText.replace("-", " ")

    # name = page_content.css('div.page-title.page-header').css('span::text').get()
    return processedSpanText


def elementBetween(container, startEle, endEle):
    containerCode = container.extract()
    startEleCode = startEle.extract()
    endEleCode = endEle.extract()

    element = None
    index = None

    return element, index


def removeIndent(code: str) -> str:
    while code.find("\n") != -1:
        code = code[:code.find("\n")] + code[code.find("\n")+1:]
    return code


def elementsBeforeHeader(page_content, header, headerElement: str) -> list:
    headerCode = "" + header.css(headerElement).extract()[0]
    endOfHeader = headerCode.index(">")
    headerCode = headerCode[:endOfHeader+1]

    elements = elementsInDiv(page_content[0])

    elementsBeforeHeader = None
    for index, element in enumerate(elements):
        if headerCode in element:
            elementsBeforeHeader = elements[:index]
            break
    for element in elementsBeforeHeader:
        elementsBeforeHeader[elementsBeforeHeader.index(
            element)] = Selector(text=element)
    return elementsBeforeHeader


def elementsBeforeList(page_content) -> list:
    extracted_page_content = page_content.extract()
    if type(extracted_page_content) == list:
        extracted_page_content = extracted_page_content[0]
    first_list_item = removeIndent(page_content.css("ul").extract()[0])
    elements = elementsInDiv(page_content[0])
    index_in_list = elements.index(first_list_item)
    elements_before_list = elements[:index_in_list]
    for index, element in enumerate(elements_before_list):
        elements_before_list[index] = Selector(text=element)
    return elements_before_list


def getParagraphsFromEleList(eleList: list) -> list:
    paragraphs = []
    for ele in eleList:
        if ele.css('p') != []:
            paragraphs.append(ele)
    return paragraphs


def combineEles(eleList: list) -> Selector:
    eleString = ""
    for ele in eleList:
        eleString += ele.extract()
    return Selector(text=eleString)


def getRaceDesc(page_content):
    header, headerElement = getHeaders(page_content)
    if header == None: 
        eleBeforeHeader = elementsBeforeList(page_content)
        subHeader = []
    else: 
        header = header[0]
        if headerElement == "h1":
            try: subHeader = header.css("h2")[0]
            except: subHeader = header.css("h2")
        else:
            try: subHeader = header.css("h3")[0]
            except: subHeader = header.css("h3")
        eleBeforeHeader = elementsBeforeHeader(page_content, header, headerElement)
    
    paragraphsBeforeHeader = getParagraphsFromEleList(eleBeforeHeader)
    if paragraphsBeforeHeader != []:
        header = combineEles(paragraphsBeforeHeader)

    strongFirst = header.css("p strong em::text").getall()
    emFirst = header.css("p em strong::text").getall()
    noEm = header.css("p strong::text").getall()

    descriptionString = "p em::text"
    if strongFirst < emFirst:
        descriptionString = "p em strong::text"
    elif noEm > strongFirst:
        descriptionString = "p strong::text"

    description = header.css(descriptionString).getall()
    if len(description) > 1 and subHeader != [] and len(paragraphsBeforeHeader) != len(description):
        description = getRaceCode(header, subHeader, headerElement).css(
            descriptionString).getall()

    for x in description:
        if "<ul>" in x and len(description) > 1:

            newDescription = []
            elements = elementsInDiv(header.css("body"))
            firstHeader = header.css("p")[0].extract()
            lastHeader = header.css("p")[-1].extract()

            elementsInBetweenDescription = elements[elements.index(
                firstHeader):elements.index(lastHeader)]
            for element in elementsInBetweenDescription:
                elementType = element[:element.index(">")+1]

                if elementType == "<p>":
                    newDescription.append(
                        Selector(text=element).css(descriptionString).get())

                elif elementType == "<ul>":
                    unorderedList = []
                    for listItem in Selector(text=element).css("li"):
                        strongText = listItem.css("strong::text").get()
                        listItemText = listItem.css("li::text").get()

                        unorderedList.append(strongText + listItemText)
                    newDescription.append(unorderedList)

            description = newDescription

            break

    return description


def getRaceFeats(page_content):
    header, headerElement = getHeaders(page_content)[0]
    subHeader = header.css("h2")

    unorderedLists = header.css("ul")
    if subHeader != []:
        unorderedLists = getRaceCode(header, subHeader, headerElement).css("ul")

    feats = featsFromList(unorderedLists)

    return feats

class Found(Exception): pass
class allRacesSpider(scrapy.Spider):
    name = "allRaces"
    start_urls = ['http://dnd5e.wikidot.com/']
    race_urls = []
    new_loop = False
    lineages = []
    count = 0
    done = False
    recordedResponses = []
    lineagesDone = []

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
                table, links = getRaceTableInfo(
                    page.css('table.wiki-content-table')[index])
                if name == "Unearthed Arcana":
                    table, links = getRaceTableInfo(
                        page.css('table.wiki-content-table')[-1])
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
                table, links = getRaceTableInfo(
                    page.css('table.wiki-content-table')[index + 4])
                for link in links:
                    self.race_urls.append(link)
                races = toList(table)
                settingSpecificLineages.append({
                    "name": name,
                    "races": races
                })
            self.lineages[-2]['lineages'] = settingSpecificLineages

            self.new_loop = True

            print(f"race_urls: {self.race_urls}")

            # yield {"race_urls": self.race_urls}

        elif self.new_loop == True:

            self.count += 1

            if self.count == len(self.race_urls)-1:
                self.done = True

            page_content = response.css("div#page-content")
            # problemRaces = ['plasmoid-ua', 'rabbitfolk-ua', 'revenant-ua', 'thri-kreen-ua', "kender", "autognome-ua", "giff-ua", "hadozee-ua", "kender-ua", "rabbitfolk-ua",]
            problemRaces = ["hobgoblin", "goblin", "centaur", "bugbear", "triton"]

            raceName = getRaceName(response)

            if "Revenant (UA)" == raceName:
                print()
                raceDesc = getRaceDesc(page_content)
                print()
            # try:
            #     raceDesc = getRaceDesc(page_content)
            # except:
            #     print("raceName: " + raceName)
                    

            thingsToDo = {
                "shifter": "has a table description",
                "glitchling-ua": "url generation fucks up"
            }

            # if ("Shifter" in raceName):
            #     webbrowser.get(using=None)
            #     webbrowser.open(response.request.url, new=2)
            #     print()

            # raceFeats = getRaceFeats(page_content)

            print()

            # raceTable = getRaceTable(page_content)
            # subraces = getRaceSubraces(page_scontent)

            # The Unearthed Arcana for Dragonborn has the source underneith
            # the name so having a check to see if the paragraph has "source" in it.

            # if page_content.css('h1').get() == None:
            #     feats = featsFromList(page_content.css("ul"))
            #     raceFeats.append(feats)
            # elif "Dwarf" in race_name:
            #     headers = splitHeaders(page_content)
            #     raceHeader = headers[0]
            #     raceFeats = featsFromList(raceHeader.css("ul"))

            #     if raceHeader.css("p") != None:
            #         if type(raceHeader.css("p")) == Selector.selectorlist_cls:
            #             for paragraph in raceHeader.css("p"):
            #                 raceDesc.append(descFromParagraph(paragraph))
            #         else:
            #             raceDesc.append(descFromParagraph(raceHeader.css("p")))
            #     if raceHeader.css("table").get() != None:
            #         raceTable = infoFromTable(raceHeader.css("table"))
            #     if len(headers) > 1:
            #         otherSources = headers[1:]

            #         for otherSource in otherSources:
            #             subraceSource = otherSource.css("h1 span::text").get()

            #             subraceHeaders = splitSubheaders(otherSource)

            #             for subraceHeader in subraceHeaders:

            #                 subraceName = subraceHeader.css(
            #                     "h2 span::text").get()
            #                 subraceFeats = featsFromList(
            #                     subraceHeader.css("ul"))
            #                 subraceDesc = []
            #                 if subraceHeader.css("p") != None:
            #                     if type(subraceHeader.css("p")) == Selector.selectorlist_cls:
            #                         for paragraph in subraceHeader.css("p"):
            #                             tempDesc, tempSource = descFromParagraph(
            #                                 paragraph)
            #                             if tempDesc != None:
            #                                 subraceDesc.append(tempDesc)
            #                             if tempSource != None:
            #                                 subraceSource = tempSource
            #                     else:
            #                         subraceDesc.append(
            #                             descFromParagraph(subraceHeader.css("p")))
            #                 subraceTable = None
            #                 if subraceHeader.css("table").get() != None:
            #                     subraceTable = infoFromTable(
            #                         subraceHeader.css("table"))

            #                 subraces.append({
            #                     "name": subraceName,
            #                     "source": subraceSource,
            #                     "desc": subraceDesc,
            #                     "feats": subraceFeats,
            #                     "table": subraceTable
            #                 })

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
            if raceName.lower() in problemRaces[0]:
                found = False
                for lineage in self.lineages:
                    if found: break
                    if lineage['name'] != "Setting Specific Lineages":
                        for race in lineage['races']:
                            if raceName.lower() == race['name'].lower():
                                race['desc'] = raceDesc
                                found = True
                                break
                    else:
                        for extraLineage in lineage['lineages']:
                            if found: break
                            for race in extraLineage['races']:
                                if raceName.lower() == race['name'].lower():
                                    race['desc'] = raceDesc
                                    found = True
                                    break
                

            # for lineage in self.lineages:
            #     try:
            #         for race in lineage['races']:
            #             if "(UA)" in raceName:
            #                 if raceName[:4].lower() in race['name'].lower() and lineage['name'] == "Unearthed Arcana":
            #                     race['desc'] = raceDesc
            #             else:
            #                 if raceName.lower() in race['name'].lower() and lineage['name'] != "Unearthed Arcana":
            #                     race['desc'] = raceDesc
            #     except:
            #         for extraLineage in lineage['lineages']:
            #             for race in extraLineage['races']:
            #                 race['desc'] = raceDesc

        for index, x in enumerate(self.race_urls):
            yield (scrapy.Request(x, callback=self.parse))
        if self.done == True:
            for lineage in self.lineages:
                if lineage['name'] not in self.lineagesDone:
                    self.lineagesDone.append(lineage['name'])
                    yield lineage


def getTag(element: Selector) -> str:
    element = element.extract()[1:element.extract().find(">")]
    return element if " " not in element else element[:element.find(" ")]

def getHeaders(element: Selector) -> list: 
    return [child for child in element.xpath('./*') if getTag(child)[0] == "h"]
    
class testSpider(scrapy.Spider):
    name = "test"
    start_urls = ['http://dnd5e.wikidot.com/lineage:revenant-ua']

    def parse(self, response):
        print(f"{'=' +'-=' * 24}\n\n\n\n")
        
        page_content = response.css("div#page-content")
        elements = page_content.xpath("./*")
        for header in getHeaders(page_content):
            print(header.get())
        # elements = page_content.xpath('./*')
        # for element in elements:
        #     print(getTag(element))
        
        print(f"\n\n\n\n{'=' +'-=' * 24}")


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
# allRacesProcess = CrawlerProcess(settings={
#     "FEEDS": {
#         "races.json": {
#             "format": "json",
#             "overwrite": True
#         }
#     }
# })
# allRacesProcess.crawl(allRacesSpider)
# allRacesProcess.start()

testProcess = CrawlerProcess(settings={
    "FEEDS": {
        "test.json": {
            "format": "json",
            "overwrite": True
        }
    }
})
testProcess.crawl(testSpider)
testProcess.start()
