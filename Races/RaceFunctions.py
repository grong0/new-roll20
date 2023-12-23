from typing import Optional
from scrapy import Selector
from scrapy.http import Response
from scrapy.selector import SelectorList


def get_lineage_table_info(
    table: Selector,
) -> tuple[list[list[dict[str, str]]], list[str]]:
    tab = []
    links = []

    for x in table.css("tr"):
        col = []
        for y in x.css("td"):
            text = x.css("td::text").get()
            link = None
            if "<a href" in y.extract():
                text = y.css("a::text").get()
                relative_link = y.css("a::attr(href)").get()
                if relative_link != None:
                    if "http://dnd5e.wikidot.com" in relative_link:
                        link = relative_link
                    else:
                        resource_path = y.css("a::attr(href)").get()
                        if resource_path != None:
                            link = "http://dnd5e.wikidot.com" + resource_path
                links.append(link)
            col.append({"name": text, "link": link})
        tab.append(col)

    for index, row in enumerate(tab):
        if row == []:
            tab.pop(index)

    return tab, links


def array_2D_to_1D(table: list[list[dict[str, str]]]) -> list[dict[str, str]]:
    tableList: list[dict[str, str]] = []
    for row in table:
        for col in row:
            if col["name"] != None:
                tableList.append(col)
    return tableList


def get_race_name(response: Response) -> str:
    page_header = response.css("div.page-title.page-header")
    span_text = page_header.css("span::text").get()

    processed_span_text = span_text.replace("-", " ")

    return processed_span_text


def get_tag(elementRaw: Selector) -> str:
    element = elementRaw.extract()[1 : elementRaw.extract().find(">")]
    return element if " " not in element else element[: element.find(" ")]


def get_headers(element: Selector) -> list[Selector]:
    return [child for child in element.xpath("./*") if get_tag(child)[0] == "h"]


def elements_before_element(parent: Selector, target: Selector) -> list[Selector]:
    children = parent.xpath("./*").getall()
    return parent.xpath("./*")[: children.index(target.get()) + 1]


def elements_between_elements(parent: Selector, start: Selector, end: Selector):
    children = parent.xpath("./*").getall()
    try:
        start_index = children.index(start.get())
    except:
        start_index = 0
    try:
        end_index = children.index(end.get()) + 1
    except:
        end_index = -1
    
    print(f"start: {start_index}")
    print(f"end: {end_index}")
    
    return parent.xpath("./*")[start_index:end_index]


def get_layout_version(response: Response) -> str:
    comments = response.xpath("//comment()")
    for comment in comments:  # type: ignore
        if "Version: " in comment.get():
            return comment.get()[
                comment.get().find(": ")
                + 2 : comment.get().find("\n", comment.get().find(": "))
            ]
    return "None"


def combine_elements(eleList: list[Selector]) -> Selector:
    eleString = ""
    for ele in eleList:
        eleString += ele.extract()
    return Selector(text=eleString).css("body")[0]


def description_string(description_content: Selector) -> str:
    strong = description_content.css("p strong em::text").getall()
    em = description_content.css("p em strong::text").getall()
    no_em = description_content.css("p strong::text").getall()

    description_string = "p em::text"
    if strong < em:
        description_string = "p em strong::text"
    elif no_em > strong:
        description_string = "p strong::text"
    return description_string


def get_children_with_children(parent: Selector, tag: str) -> SelectorList:
    children = parent.xpath("./*")
    valid_children = []
    for child in children:
        children_with_tag = child.xpath(".//" + tag)
        if len(children_with_tag) != 0:
            valid_children.append(child)
    return SelectorList(valid_children)


def interlace_list_at_index(parent: list, appender: list, starting_index: int) -> list:
    for index, item in enumerate(appender):
        parent.insert(starting_index + (index * 2), item)
    return parent


def concat_str_elements_at_index(parent: list, starting_index: int) -> list:
    concatenated_str = ""
    for item in parent[starting_index:]:
        concatenated_str += item
    parent = parent[:starting_index]
    parent.append(concatenated_str)
    return parent


def table_elements_to_2D_array(table: Selector) -> tuple[Optional[str], Optional[list[str]], Optional[list[list[str]]]]:  # type: ignore
    name = None
    headers = table.css("th::text").getall()
    content: list[list[str]] = []
    content_index = 1

    # If the table has a name
    if len(table.css("tr")[0].css("th")) == 1:
        name = table.css("th::text").get()
        headers = table.css("th::text").getall()[1:]
        content_index = 2

    # Getting Content
    for row in table.css("tr")[content_index:]:
        content.append(row.css("td::text").getall())

    return name, headers, content
