from scrapy import Selector
from scrapy.http import Response


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
