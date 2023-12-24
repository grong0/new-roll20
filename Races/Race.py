from prettyprinter import pprint
from scrapy import Selector
from scrapy.http import Response

from RaceFunctions import (
    combine_elements,
    description_string,
    elements_after_element,
    elements_before_element,
    elements_between_elements,
    get_elements_contents,
    get_highest_header,
    get_layout_version,
    get_tag,
    table_elements_to_2D_array,
)


class Race:
    name: str
    link: str
    source: str
    description: list[str]
    feats: list[
        dict[str, str | int | dict[str, str | list[str] | list[list[str]] | None]]
    ]
    sub_feats: list[dict[str, str | int]]  # "text": "", "index" 0, "depth": 0
    sub_races: list["Race"]

    def __init__(self, name: str, link: str) -> None:
        self.name = name
        self.link = link
        self.source = ""
        self.description = []
        self.feats = []

    def compile_race(self, response: Response) -> None:
        page_content = response.css("div#page-content")

        version = get_layout_version(response)
        source_elements = elements_before_element(
            page_content, page_content.css("h1")[1]
        )[:-1]
        source_content = combine_elements(source_elements)
        if source_content.css("h2").get() != None:
            source_elements = elements_before_element(
                source_content, source_content.css("h2")[0]
            )
            source_content = combine_elements(source_elements)

        if self.source == "":
            self.source = self.extract_source(source_content, version)

        self.description = self.extract_description(source_content, version)

        self.feats = self.extract_feats(source_content, version)

        self.compile_subraces(response)

        print()

    def compile_subraces(self, response: Response):
        page_content = response.css("div#page-content")

        sources = get_elements_contents(page_content, "h1")
        for source in sources[1:]:
            subraces = get_elements_contents(source, "h2")

            for subrace in subraces:
                name = subrace.css("h2 span::text").get()
                link = self.link + "#" + subrace.css("h2::attr(id)").get()
                subrace_source = source.css("span::text").get()
                description = self.extract_description(
                    subrace, get_layout_version(response)
                )

                print(f"name: {name}")
                print(f"link: {link}")
                print(f"source: {subrace_source}")
                print(f"description: {description}")
                print()

    def extract_source(self, source_content: Selector, version: str) -> str:
        source = ""

        if version == "1.01.141118":
            source_response = source_content.css("h1 span::text").get()
            if source_response != None:
                source = source_response
        else:
            pass

        return source

    def extract_description(self, source_content: Selector, version: str) -> list[str]:
        description: list[str] = [""]

        if version == "1.01.141118":
            description_elements = elements_between_elements(
                source_content,
                source_content.css(get_highest_header(source_content))[0],
                source_content.css("p")[-1],
            )[1:]
            description_content = combine_elements(description_elements)
            description = description_content.css(
                description_string(description_content)
            ).getall()
        else:
            pass

        return description

    def extract_feats(
        self, source_content: Selector, version: str
    ) -> list[
        dict[str, str | int | dict[str, str | list[str] | list[list[str]] | None]]
    ]:
        feats: list[
            dict[str, str | int | dict[str, str | list[str] | list[list[str]] | None]]
        ] = []

        if version == "1.01.141118":
            feat_elements = elements_between_elements(
                source_content,
                source_content.xpath("./ul")[0],
                source_content.xpath("./ul")[-1],
            )
            for element in feat_elements:
                if get_tag(element) == "ul":
                    feat_name = element.css("li strong::text").get()
                    feat_description = element.css("li::text").get()

                    if feat_name != None and feat_description != None:
                        feats.append(
                            {
                                "name": feat_name.removesuffix("."),
                                "description": feat_description.removeprefix(" "),
                                "indent": 0,
                            }
                        )

                    # If there are sub feats
                    if element.css("ul li ul").get() != None:
                        sub_feats = element.css("ul li ul li")
                        for sub_feat in sub_feats:
                            sub_feat_name = sub_feat.css("strong::text").get()
                            sub_feat_description = sub_feat.xpath("./text()").get()

                            # If the sub feat is at a lower level
                            if sub_feat.css("ul").get() != None:
                                for subber_feat in sub_feat.css("ul li"):
                                    subber_feat_name = subber_feat.css(
                                        "strong::text"
                                    ).get()
                                    subber_feat_description = subber_feat.xpath(
                                        "./text()"
                                    ).get()

                                    if (
                                        subber_feat_name != None
                                        and subber_feat_description != None
                                    ):
                                        feats.append(
                                            {
                                                "name": subber_feat_name.removesuffix(
                                                    "."
                                                ),
                                                "description": subber_feat_description.removeprefix(
                                                    " "
                                                ),
                                                "indent": 2,
                                            }
                                        )

                            # If the sub feat is at the next lower level
                            elif sub_feat_name != None and sub_feat_description != None:
                                feats.append(
                                    {
                                        "name": sub_feat_name.removesuffix("."),
                                        "description": sub_feat_description.removeprefix(
                                            " "
                                        ),
                                        "indent": 1,
                                    }
                                )

                elif get_tag(element) == "table":
                    name, headers, content = table_elements_to_2D_array(element)
                    feats[-1]["table"] = {
                        "name": name,
                        "headers": headers,
                        "content": content,
                    }

        return feats

    def get_as_dict(
        self,
    ) -> dict[
        str,
        str
        | list[str]
        | list[
            dict[str, str | int | dict[str, str | list[str] | list[list[str]] | None]]
        ],
    ]:
        return {
            "name": self.name,
            "link": self.link,
            "source": self.source,
            "description": self.description,
            "feats": self.feats,
            # "sub_feats": self.sub_feats,
            # "sub_races": self.sub_races,
        }
