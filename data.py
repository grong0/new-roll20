import string
from typing import Optional
from parsel import SelectorList
from pkg_resources import require
import scrapy
from scrapy.crawler import CrawlerProcess
from scrapy.selector import Selector
import webbrowser
import os
from colorama import Fore


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


def indexFromPoint(txt: str, char: str, index: int) -> Optional[int]:
    for pos, x in enumerate(txt):
        if pos < index:
            continue
        if txt[pos:pos+len(char)] == char:
            return pos
    return None


def indexInList(list, str):
    for index, x in enumerate(list):
        if str in x:
            return index


def removeNothing(list: list) -> list:
    for item in list:
        if item == "":
            list.remove(item)
    return list



# def elementsInDiv(div: Selector) -> list:
#     code: str = div.extract()
#     main_element = code[1:code.find(">")]
#     if " " in main_element: main_element = main_element[:main_element.find(" ")]
#     code = removeIndent(code)[code.find(">")+1:-len(main_element)-3]

#     new_elements = []
    
#     looking = False
#     sub_children = 0
#     target = {
#         "ele": "",
#         "index": 0,
#     }
    
#     for index, char in enumerate(code):
#         if char == "<":
#             element = code[index:code.find(">", index)+1]
#             if "<br" in element: continue
#             if "/" in element and looking:
#                 if sub_children == 0:
#                     new_elements.append(code[target['index']:index+len(target['ele'])+3])
#                     looking = False
#                 else: sub_children -= 1
#             else:
#                 if looking: sub_children += 1
#                 else:
#                     target = {
#                         "ele": code[index+1:code.find(">", index)],
#                         "index": index
#                     }
#                     looking = True
#     return new_elements


def getRaceTableInfo(table) -> tuple[list, list]:
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
    

def getElementName(element: Selector) -> str:
    """The name of the element

    Args:
        element (str): an element's code1

    Returns:
        str: the element's name
    """
    elementCode: str = element.extract();
    if "/" in elementCode: inside = elementCode[2:-1]
    else: inside = elementCode[1:-1]
    return inside[:inside.find(" ")]


def getHeadersWithinDiv(div: Selector) -> list:
    """Header elements within an element

    Args:
        div (Selector): element to search in

    Returns:
        list: list of hearders
    """
    headers = []
    for element in div.xpath("./"):
        if len(getElementName(element)) and "h" in getElementName(element):
            headers.append(element)
    return headers


# def getHeaders(page_content):
    
#     pageContentCode = page_content.extract()[0].replace("\n", "")
#     headers = page_content.css("h1")
#     headerElement = "h1"
#     if len(headers) == 0: 
#         headers = page_content.css("h2")
#         headerElement = "h2"
#         if len(headers) == 0:
#             return None, None
#     headersCode = headers.extract()

#     if len(headersCode) == 1:
#         return page_content, headerElement

#     headersCodeContent = []
#     for index in range(0, len(headersCode)-1):
#         currentHeaderIndex = pageContentCode.index(headersCode[index])
#         nextHeaderIndex = pageContentCode.index(headersCode[index+1])
#         currentHeaderCodeContent = pageContentCode[currentHeaderIndex:nextHeaderIndex]
#         headersCodeContent.append(currentHeaderCodeContent)
#     indexOfLastHeader = pageContentCode.index(headersCode[-1])
#     indexOfDiv = indexFromPoint(pageContentCode, "</div>", indexOfLastHeader)
#     headersCodeContent.append(pageContentCode[indexOfLastHeader:indexOfDiv])

#     headersContent = []
#     for headerCodeContent in headersCodeContent:
#         headerContent = Selector(text=headerCodeContent)
#         headersContent.append(headerContent)

#     return headersContent, headerElement


# def getSubHeaders(header, headerElement):
#     headerContentCode = header.extract()[0]
#     subHeaders = header.css(headerElement)
#     subHeadersCode = subHeaders.extract()

#     if len(subHeadersCode) == 1:
#         return None

#     subHeadersCodeContent = []
#     for index in range(0, len(subHeadersCode)-1):
#         currentHeaderIndex = headerContentCode.index(subHeadersCode[index])
#         nextHeaderIndex = headerContentCode.index(subHeadersCode[index])
#         currentHeaderCodeContent = headerContentCode[currentHeaderIndex:nextHeaderIndex]
#         subHeadersCodeContent.append(currentHeaderCodeContent)

#     subHeadersContent = []
#     for subHeaderCodeContent in subHeadersContent:
#         subHeaderContent = Selector(text=subHeaderCodeContent)
#         subHeadersContent.append(subHeaderContent)

#     return subHeadersContent


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

    if idexOfEndOfHeader != None:
        mainRaceCode = headerCode[idexOfEndOfHeader+1:indexOfSubHeader]
    else: return Selector(text="")
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

    elements: SelectorList = page_content[0].xpath("./*")

    elementsBeforeHeader = SelectorList()
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
    elements = page_content[0].xpath("./*")
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





def descriptionString(descriptionContent: Selector) -> str:
    strongFirst = descriptionContent.css("p strong em::text").getall()
    emFirst = descriptionContent.css("p em strong::text").getall()
    noEm = descriptionContent.css("p strong::text").getall()
    
    descriptionString = "p em::text"
    if strongFirst < emFirst:
        descriptionString = "p em strong::text"
    elif noEm > strongFirst:
        descriptionString = "p strong::text"
    return descriptionString


def getRaceDesc(page_content: Selector, layout_version = None):
    # elementsBeforeHeader = elementsBeforeElement(page_content, page_content.css("p")[-1])
    
    # paragraphsBeforeHeader = getParagraphsFromEleList(elementsBeforeHeader)
    # if paragraphsBeforeHeader != []:
    #     descriptionContent = combineEles(paragraphsBeforeHeader)

    # description = descriptionContent.css(descriptionLocation(descriptionContent)).getall()

    # for index, x in enumerate(description):
    #     if "<ul>" in x and len(description) > 1:
            
            
            
            
            # description.insert(index, )
            

            # newDescription = []
            # elements = elementsInDiv(header.css("body"))
            # firstHeader = header.css("p")[0].extract()
            # lastHeader = header.css("p")[-1].extract()

            # elementsInBetweenDescription = elements[elements.index(
            #     firstHeader):elements.index(lastHeader)]
            # for element in elementsInBetweenDescription:
            #     elementType = element[:element.index(">")+1]

            #     if elementType == "<p>":
            #         newDescription.append(
            #             Selector(text=element).css(descriptionString).get())

            #     elif elementType == "<ul>":
            #         unorderedList = []
            #         for listItem in Selector(text=element).css("li"):
            #             strongText = listItem.css("strong::text").get()
            #             listItemText = listItem.css("li::text").get()

            #             unorderedList.append(strongText + listItemText)
            #         newDescription.append(unorderedList)

            # description = newDescription

            # break
            
    if layout_version == "1.01.141118":
        source_elements = elementsBeforeElement(page_content, page_content.css("h1")[1])[:-1]
        source_content = combineEles(source_elements)
        race_elments = elementsBeforeElement(source_content, source_content.css("h2")[0])
        race_content = combineEles(race_elments)
        
        description_elements = elementsBetweenElements(race_content, race_content.css("#toc0")[0], race_content.css("p")[-1])[1:]
        description_content = combineEles(description_elements)
        description = description_content.css(descriptionString(description_content)).getall()
    else: description = None
    print(description)

    return description


# def getRaceFeats(page_content):
#     header, headerElement = getHeaders(page_content)
#     subHeader = header.css("h2")

#     unorderedLists = header.css("ul")
#     if subHeader != []:
#         unorderedLists = getRaceCode(header, subHeader, headerElement).css("ul")

#     feats = featsFromList(unorderedLists)

#     return feats

class Found(Exception): pass
class allRacesSpider(scrapy.Spider):
    name = "allRaces"
    start_urls = ['http://dnd5e.wikidot.com/']
    race_urls = []
    finished_races = []
    new_loop = False
    lineages = []
    count = 0
    done = False
    finishedYielding = False
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

            yield {"race_urls": self.race_urls}

        elif self.new_loop == True:

            self.count += 1

            if self.count == len(self.race_urls)-2:
                self.done = True

            page_content = response.css("div#page-content")
            # problemRaces = ['plasmoid-ua', 'rabbitfolk-ua', 'revenant-ua', 'thri-kreen-ua', "kender", "autognome-ua", "giff-ua", "hadozee-ua", "kender-ua", "rabbitfolk-ua",]
            problemRaces = ["hobgoblin", "goblin", "centaur", "bugbear", "triton"]

            layout_version = getLayoutVersion(response)

            raceName = getRaceName(response)
            self.finished_races.append(response.request.url)
            
            if raceName == "Kender":
                print("\n\n\n\n\nKENDER HAS BEEN FOUND\n\n\n\n\n")
            
            try:
                raceDesc = getRaceDesc(page_content, layout_version)
            except Exception as exception:
                raceDesc = exception
            
            print()

            # if "Revenant (UA)" == raceName:
            #     print()
            #     raceDesc = getRaceDesc(page_content)
            #     print()
            # try:
            #     raceDesc = getRaceDesc(page_content)
            # except:
            #     print("raceName: " + raceName)
                    

            # thingsToDo = {
            #     "shifter": "has a table description",
            #     "glitchling-ua": "url generation fucks up"
            # }

            # if ("Shifter" in raceName):
            #     webbrowser.get(using=None)
            #     webbrowser.open(response.request.url, new=2)
            #     print()

            # raceFeats = getRaceFeats(page_content)

            # print()

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
            # if raceName.lower() in problemRaces[0]:
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
            
                    
    
def getTag(elementRaw: Selector) -> str:
    element = elementRaw.extract()[1:elementRaw.extract().find(">")]
    return element if " " not in element else element[:element.find(" ")]

def getHeaders(element: Selector) -> list[Selector]:
    return [child for child in element.xpath('./*') if getTag(child)[0] == "h"]

def elementsBeforeElement(parent: Selector, target: Selector) -> list[Selector]:
    children = parent.xpath('./*').getall()
    return parent.xpath('./*')[:children.index(target.get())+1]

def elementsBetweenElements(parent: Selector, start: Selector, end: Selector):
    children = parent.xpath('./*').getall()
    return parent.xpath('./*')[children.index(start.get()):children.index(end.get())+1]

def getLayoutVersion(response: Selector) -> str:
    comments = response.xpath("//comment()")
    for comment in comments:
        if "Version: " in comment.get():
            return comment.get()[comment.get().find(": ") + 2:comment.get().find("\n", comment.get().find(": "))]
    return "None"

def combineEles(eleList: list[Selector]) -> Selector:
    eleString = ""
    for ele in eleList:
        eleString += ele.extract()
    return Selector(text=eleString).css("body")[0]

def getChildrenThatHaveChildren(parent: Selector, tag: str) -> SelectorList[Selector]:
    children = parent.xpath("./*")
    valid_children = []
    for child in children:
        children_with_tag = child.xpath(".//" + tag)
        if len(children_with_tag) != 0:
            valid_children.append(child)
    return SelectorList(valid_children)

def interlaceListAtIndex(parent: list, appender: list, starting_index: int) -> list:
    for index, item in enumerate(appender):
        parent.insert(starting_index + (index * 2), item)
    return parent

def concatStrElementsAtIndex(parent: list, starting_index: int) -> list:
    concatenated_str = ""
    for item in parent[starting_index:]: concatenated_str += item
    parent = parent[:starting_index]
    parent.append(concatenated_str)
    return parent

def tableElementTo2DArray(table: Selector) -> tuple[any, list[list[str]]]: # type: ignore
    headers = table.xpath(".//tbody").extract()
    content = []

    # for row in table.xpath(".//tbody/tr"):
    #     list_row = []
    #     for col in row.xpath("")

    return headers, content

class allClassesSpider(scrapy.Spider):
    name = "allClasses"
    start_urls = ["http://dnd5e.wikidot.com/"]
    class_urls = []
    classes = {}
    new_loop = True
    race_start = 7
    race_end = 20
    ran_ending = False
    
    def parse(self, response):
        if self.new_loop:
            page_content = response.css("#page-content").xpath("./*")[1]
            for class_feature in page_content.xpath("./*")[self.race_start-1:self.race_end]:
                class_bio = class_feature.css(".feature")[0]
                
                name = class_bio.css("a::text").get()
                key = name.lower() if "(" not in name else name[:name.find("(")-1].lower()
                link = 'http://dnd5e.wikidot.com' + class_bio.css("a::attr(href)").get()

                self.classes[key] = {
                    "name": name,
                    "link": link
                }
                self.class_urls.append(link)
            self.new_loop = False
        else:
            page_content = response.css("#page-content")
            bio_content = elementsBeforeElement(page_content, page_content.css("table")[0])
            bio_content = combineEles(bio_content)
            
            # Getting the name
            name = response.css(".page-title span::text").get()
            
            # Getting the description
            description = []
            for element in bio_content.xpath(".//p"):
                if element.css("p::text").get() == None and element.xpath(".//strong/em/text()").get() != None:
                    description.append(element.xpath(".//strong/em/text()").get())
            
            # Getting the specific requirements + extra notes on class compatibility
            requirement_elements = bio_content.xpath(".//p/em")
            requirement_elements_code = [element.get() for element in requirement_elements]
            requirements = [element.xpath("./text()").get() for element in requirement_elements]
            for index, element in enumerate(requirement_elements_code):
                if "</a>" in element:
                    em_text = bio_content.xpath(".//p/em/text()").getall()
                    a_text = bio_content.xpath(".//p/em/a/text()").getall()
                    requirements = concatStrElementsAtIndex(interlaceListAtIndex(em_text, a_text, index+1), index)
                
            # Getting the table data
            table = tableElementTo2DArray(bio_content.css("table.wiki-content-table")[0])


                
            print("\n\n\n")
            print("name: " + name)
            print("description: " + str(description))
            print("requirements: " + str(requirements))
            print(f"table: {table}")
            # print("\n\n\n")
            
            yield self.classes[response.css(".page-title span::text").get().lower()]
        for index, x in enumerate(self.class_urls):
            yield (scrapy.Request(x, callback=self.parse))
            
    def spider_idle(self):
        print("\n\n\nclosed happened\n\n\n")
        yield self.classes
            

class testSpider(scrapy.Spider):
    name = "test"
    start_urls = ['http://dnd5e.wikidot.com/lineage:dragonborn']

    def parse(self, response):
        print(f"{'=' +'-=' * 24}\n\n\n\n")
        
        try:
            if getLayoutVersion(response) == "1.01.141118":
                page_content = response.css("div#page-content")
                source_elements = elementsBeforeElement(page_content, page_content.css("h1")[1])[:-1]
                source_content = combineEles(source_elements)
                if (source_content.css("h2").get() != None):
                    race_elements = elementsBeforeElement(source_content, source_content.css("h2")[0])
                    race_content = combineEles(race_elements)
                else:
                    race_content = source_content
                
                description_elements = elementsBetweenElements(race_content, race_content.css("#toc0")[0], race_content.css("p")[-1])[1:]
                description_content = combineEles(description_elements)
                description = description_content.css(descriptionString(description_content)).getall()
            else: description = None
            print(description)

            # print(descrption_content)
            
        except Exception as e:
            print(f"{Fore.RED}{e}{Fore.WHITE}")
        
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

allClassesProcess = CrawlerProcess(settings={
    "FEEDS": {
        "classes.json": {
            "format": "json",
            "overwrite": True
        }
    }
})
allClassesProcess.crawl(allClassesSpider)
allClassesProcess.start()

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
