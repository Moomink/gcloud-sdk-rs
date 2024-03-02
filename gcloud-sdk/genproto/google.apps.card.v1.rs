/// A card interface displayed in a Google Chat message or Google Workspace
/// Add-on.
///
/// Cards support a defined layout, interactive UI elements like buttons, and
/// rich media like images. Use cards to present detailed information,
/// gather information from users, and guide users to take a next step.
///
/// [Card builder](<https://addons.gsuite.google.com/uikit/builder>)
///
/// To learn how
/// to build cards, see the following documentation:
///
/// * For Google Chat apps, see [Design dynamic, interactive, and consistent UIs
/// with cards](<https://developers.google.com/chat/ui>).
/// * For Google Workspace Add-ons, see [Card-based
/// interfaces](<https://developers.google.com/apps-script/add-ons/concepts/cards>).
///
/// **Example: Card message for a Google Chat app**
///
/// ![Example contact
/// card](<https://developers.google.com/chat/images/card_api_reference.png>)
///
/// To create the sample card message in Google Chat, use the following JSON:
///
/// ```
/// {
///    "cardsV2": [
///      {
///        "cardId": "unique-card-id",
///        "card": {
///          "header": {
///            "title": "Sasha",
///            "subtitle": "Software Engineer",
///            "imageUrl":
///            "<https://developers.google.com/chat/images/quickstart-app-avatar.png",>
///            "imageType": "CIRCLE",
///            "imageAltText": "Avatar for Sasha",
///          },
///          "sections": [
///            {
///              "header": "Contact Info",
///              "collapsible": true,
///              "uncollapsibleWidgetsCount": 1,
///              "widgets": [
///                {
///                  "decoratedText": {
///                    "startIcon": {
///                      "knownIcon": "EMAIL",
///                    },
///                    "text": "sasha@example.com",
///                  }
///                },
///                {
///                  "decoratedText": {
///                    "startIcon": {
///                      "knownIcon": "PERSON",
///                    },
///                    "text": "<font color=\"#80e27e\">Online</font>",
///                  },
///                },
///                {
///                  "decoratedText": {
///                    "startIcon": {
///                      "knownIcon": "PHONE",
///                    },
///                    "text": "+1 (555) 555-1234",
///                  }
///                },
///                {
///                  "buttonList": {
///                    "buttons": [
///                      {
///                        "text": "Share",
///                        "onClick": {
///                          "openLink": {
///                            "url": "<https://example.com/share",>
///                          }
///                        }
///                      },
///                      {
///                        "text": "Edit",
///                        "onClick": {
///                          "action": {
///                            "function": "goToView",
///                            "parameters": [
///                              {
///                                "key": "viewType",
///                                "value": "EDIT",
///                              }
///                            ],
///                          }
///                        }
///                      },
///                    ],
///                  }
///                },
///              ],
///            },
///          ],
///        },
///      }
///    ],
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Card {
    /// The header of the card. A header usually contains a leading image and a
    /// title. Headers always appear at the top of a card.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<card::CardHeader>,
    /// Contains a collection of widgets. Each section has its own, optional
    /// header. Sections are visually separated by a line divider. For an example
    /// in Google Chat apps, see [Card
    /// section](<https://developers.google.com/chat/ui/widgets/card-section>).
    #[prost(message, repeated, tag = "2")]
    pub sections: ::prost::alloc::vec::Vec<card::Section>,
    /// The divider style between sections.
    #[prost(enumeration = "card::DividerStyle", tag = "9")]
    pub section_divider_style: i32,
    /// The card's actions. Actions are added to the card's toolbar menu.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    ///
    /// For example, the following JSON constructs a card action menu with
    /// `Settings` and `Send Feedback` options:
    ///
    /// ```
    /// "card_actions": [
    ///    {
    ///      "actionLabel": "Settings",
    ///      "onClick": {
    ///        "action": {
    ///          "functionName": "goToView",
    ///          "parameters": [
    ///            {
    ///              "key": "viewType",
    ///              "value": "SETTING"
    ///           }
    ///          ],
    ///          "loadIndicator": "LoadIndicator.SPINNER"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "actionLabel": "Send Feedback",
    ///      "onClick": {
    ///        "openLink": {
    ///          "url": "<https://example.com/feedback">
    ///        }
    ///      }
    ///    }
    /// ]
    /// ```
    #[prost(message, repeated, tag = "3")]
    pub card_actions: ::prost::alloc::vec::Vec<card::CardAction>,
    /// Name of the card. Used as a card identifier in card navigation.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The fixed footer shown at the bottom of this card.
    ///
    /// Setting `fixedFooter` without specifying a `primaryButton` or a
    /// `secondaryButton` causes an error. For Chat apps, you can use fixed footers
    /// in
    /// [dialogs](<https://developers.google.com/chat/how-tos/dialogs>), but not
    /// [card
    /// messages](<https://developers.google.com/chat/api/guides/v1/messages/create#create>).
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[prost(message, optional, boxed, tag = "5")]
    pub fixed_footer: ::core::option::Option<
        ::prost::alloc::boxed::Box<card::CardFixedFooter>,
    >,
    /// In Google Workspace Add-ons, sets the display properties of the
    /// `peekCardHeader`.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[prost(enumeration = "card::DisplayStyle", tag = "6")]
    pub display_style: i32,
    /// When displaying contextual content, the peek card header acts as a
    /// placeholder so that the user can navigate forward between the homepage
    /// cards and the contextual cards.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[prost(message, optional, tag = "7")]
    pub peek_card_header: ::core::option::Option<card::CardHeader>,
}
/// Nested message and enum types in `Card`.
pub mod card {
    /// Represents a card header. For an example in Google Chat apps, see [Card
    /// header](<https://developers.google.com/chat/ui/widgets/card-header>).
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CardHeader {
        /// Required. The title of the card header.
        /// The header has a fixed height: if both a
        /// title and subtitle are specified, each takes up one line. If only the
        /// title is specified, it takes up both lines.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The subtitle of the card header. If specified, appears on its own line
        /// below the `title`.
        #[prost(string, tag = "2")]
        pub subtitle: ::prost::alloc::string::String,
        /// The shape used to crop the image.
        ///
        /// [Google Workspace Add-ons and Chat
        /// apps](<https://developers.google.com/workspace/extend>):
        #[prost(enumeration = "super::widget::ImageType", tag = "3")]
        pub image_type: i32,
        /// The HTTPS URL of the image in the card header.
        #[prost(string, tag = "4")]
        pub image_url: ::prost::alloc::string::String,
        /// The alternative text of this image that's used for accessibility.
        #[prost(string, tag = "5")]
        pub image_alt_text: ::prost::alloc::string::String,
    }
    /// A section contains a collection of widgets that are rendered
    /// vertically in the order that they're specified.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Section {
        /// Text that appears at the top of a section.
        /// Supports simple HTML formatted text. For more information
        /// about formatting text, see
        /// [Formatting text in Google Chat
        /// apps](<https://developers.google.com/chat/format-messages#card-formatting>)
        /// and
        /// [Formatting
        /// text in Google Workspace
        /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
        #[prost(string, tag = "1")]
        pub header: ::prost::alloc::string::String,
        /// All the widgets in the section.
        /// Must contain at least one widget.
        #[prost(message, repeated, tag = "2")]
        pub widgets: ::prost::alloc::vec::Vec<super::Widget>,
        /// Indicates whether this section is collapsible.
        ///
        /// Collapsible sections hide some or all widgets, but users can expand the
        /// section to reveal the hidden widgets by clicking **Show more**. Users
        /// can hide the widgets again by clicking **Show less**.
        ///
        /// To determine which widgets are hidden, specify
        /// `uncollapsibleWidgetsCount`.
        #[prost(bool, tag = "5")]
        pub collapsible: bool,
        /// The number of uncollapsible widgets which remain visible even when a
        /// section is collapsed.
        ///
        /// For example, when a section
        /// contains five widgets and the `uncollapsibleWidgetsCount` is set to `2`,
        /// the first two widgets are always shown and the last three are collapsed
        /// by default. The `uncollapsibleWidgetsCount` is taken into account only
        /// when `collapsible` is `true`.
        #[prost(int32, tag = "6")]
        pub uncollapsible_widgets_count: i32,
    }
    /// A card action is the action associated with the card. For example,
    /// an invoice card might include actions such as delete invoice, email
    /// invoice, or open the invoice in a browser.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CardAction {
        /// The label that displays as the action menu item.
        #[prost(string, tag = "1")]
        pub action_label: ::prost::alloc::string::String,
        /// The `onClick` action for this action item.
        #[prost(message, optional, tag = "2")]
        pub on_click: ::core::option::Option<super::OnClick>,
    }
    /// A persistent (sticky) footer that that appears at the bottom of the card.
    ///
    /// Setting `fixedFooter` without specifying a `primaryButton` or a
    /// `secondaryButton` causes an error.
    ///
    /// For Chat apps, you can use fixed footers in
    /// [dialogs](<https://developers.google.com/chat/how-tos/dialogs>), but not
    /// [card
    /// messages](<https://developers.google.com/chat/api/guides/v1/messages/create#create>).
    /// For an example in Google Chat apps, see [Card
    /// footer](<https://developers.google.com/chat/ui/widgets/card-fixed-footer>).
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CardFixedFooter {
        /// The primary button of the fixed footer. The button must be a text button
        /// with text and color set.
        #[prost(message, optional, boxed, tag = "1")]
        pub primary_button: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Button>,
        >,
        /// The secondary button of the fixed footer.  The button must be a text
        /// button with text and color set.
        /// If `secondaryButton` is set, you must also set `primaryButton`.
        #[prost(message, optional, boxed, tag = "2")]
        pub secondary_button: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Button>,
        >,
    }
    /// The divider style of a card. Currently only used for dividers betweens card
    /// sections.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DividerStyle {
        /// Don't use. Unspecified.
        Unspecified = 0,
        /// Default option. Render a solid divider between sections.
        SolidDivider = 1,
        /// If set, no divider is rendered between sections.
        NoDivider = 2,
    }
    impl DividerStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DividerStyle::Unspecified => "DIVIDER_STYLE_UNSPECIFIED",
                DividerStyle::SolidDivider => "SOLID_DIVIDER",
                DividerStyle::NoDivider => "NO_DIVIDER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DIVIDER_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
                "SOLID_DIVIDER" => Some(Self::SolidDivider),
                "NO_DIVIDER" => Some(Self::NoDivider),
                _ => None,
            }
        }
    }
    /// In Google Workspace Add-ons,
    /// determines how a card is displayed.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DisplayStyle {
        /// Don't use. Unspecified.
        Unspecified = 0,
        /// The header of the card appears at the bottom of the
        /// sidebar, partially covering the current top card of the stack. Clicking
        /// the header pops the card into the card stack. If the card has no header,
        /// a generated header is used instead.
        Peek = 1,
        /// Default value. The card is shown by replacing the view of the top card in
        /// the card stack.
        Replace = 2,
    }
    impl DisplayStyle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DisplayStyle::Unspecified => "DISPLAY_STYLE_UNSPECIFIED",
                DisplayStyle::Peek => "PEEK",
                DisplayStyle::Replace => "REPLACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISPLAY_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
                "PEEK" => Some(Self::Peek),
                "REPLACE" => Some(Self::Replace),
                _ => None,
            }
        }
    }
}
/// Each card is made up of widgets.
///
/// A widget is a composite object that can represent one of text, images,
/// buttons, and other object types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Widget {
    /// Specifies whether widgets align to the left, right, or center of a column.
    #[prost(enumeration = "widget::HorizontalAlignment", tag = "8")]
    pub horizontal_alignment: i32,
    /// A widget can only have one of the following items. You can use multiple
    /// widget fields to display more items.
    #[prost(oneof = "widget::Data", tags = "1, 2, 3, 4, 5, 6, 7, 9, 10, 11")]
    pub data: ::core::option::Option<widget::Data>,
}
/// Nested message and enum types in `Widget`.
pub mod widget {
    /// The shape used to crop the image.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ImageType {
        /// Default value. Applies a square mask to the image. For example, a 4x3
        /// image becomes 3x3.
        Square = 0,
        /// Applies a circular mask to the image. For example, a 4x3 image becomes a
        /// circle with a diameter of 3.
        Circle = 1,
    }
    impl ImageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImageType::Square => "SQUARE",
                ImageType::Circle => "CIRCLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQUARE" => Some(Self::Square),
                "CIRCLE" => Some(Self::Circle),
                _ => None,
            }
        }
    }
    /// Specifies whether widgets align to the left, right, or center of a column.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum HorizontalAlignment {
        /// Don't use. Unspecified.
        Unspecified = 0,
        /// Default value. Aligns widgets to the start position of the column. For
        /// left-to-right layouts, aligns to the left. For right-to-left layouts,
        /// aligns to the right.
        Start = 1,
        /// Aligns widgets to the center of the column.
        Center = 2,
        /// Aligns widgets to the end position of the column. For left-to-right
        /// layouts, aligns widgets to the right. For right-to-left layouts, aligns
        /// widgets to the left.
        End = 3,
    }
    impl HorizontalAlignment {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HorizontalAlignment::Unspecified => "HORIZONTAL_ALIGNMENT_UNSPECIFIED",
                HorizontalAlignment::Start => "START",
                HorizontalAlignment::Center => "CENTER",
                HorizontalAlignment::End => "END",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => Some(Self::Unspecified),
                "START" => Some(Self::Start),
                "CENTER" => Some(Self::Center),
                "END" => Some(Self::End),
                _ => None,
            }
        }
    }
    /// A widget can only have one of the following items. You can use multiple
    /// widget fields to display more items.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Displays a text paragraph. Supports simple HTML formatted text. For more
        /// information about formatting text, see
        /// [Formatting text in Google Chat
        /// apps](<https://developers.google.com/chat/format-messages#card-formatting>)
        /// and
        /// [Formatting
        /// text in Google Workspace
        /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
        ///
        /// For example, the following JSON creates a bolded text:
        /// ```
        /// "textParagraph": {
        ///    "text": "  <b>bold text</b>"
        /// }
        /// ```
        #[prost(message, tag = "1")]
        TextParagraph(super::TextParagraph),
        /// Displays an image.
        ///
        /// For example, the following JSON creates an image with alternative text:
        /// ```
        /// "image": {
        ///    "imageUrl":
        ///    "<https://developers.google.com/chat/images/quickstart-app-avatar.png",>
        ///    "altText": "Chat app avatar"
        /// }
        /// ```
        #[prost(message, tag = "2")]
        Image(super::Image),
        /// Displays a decorated text item.
        ///
        /// For example, the following JSON creates a decorated text widget showing
        /// email address:
        ///
        /// ```
        /// "decoratedText": {
        ///    "icon": {
        ///      "knownIcon": "EMAIL"
        ///    },
        ///    "topLabel": "Email Address",
        ///    "text": "sasha@example.com",
        ///    "bottomLabel": "This is a new Email address!",
        ///    "switchControl": {
        ///      "name": "has_send_welcome_email_to_sasha",
        ///      "selected": false,
        ///      "controlType": "CHECKBOX"
        ///    }
        /// }
        /// ```
        #[prost(message, tag = "3")]
        DecoratedText(super::DecoratedText),
        /// A list of buttons.
        ///
        /// For example, the following JSON creates two buttons. The first
        /// is a blue text button and the second is an image button that opens a
        /// link:
        /// ```
        /// "buttonList": {
        ///    "buttons": [
        ///      {
        ///        "text": "Edit",
        ///        "color": {
        ///          "red": 0,
        ///          "green": 0,
        ///          "blue": 1,
        ///          "alpha": 1
        ///        },
        ///        "disabled": true,
        ///      },
        ///      {
        ///        "icon": {
        ///          "knownIcon": "INVITE",
        ///          "altText": "check calendar"
        ///        },
        ///        "onClick": {
        ///          "openLink": {
        ///            "url": "<https://example.com/calendar">
        ///          }
        ///        }
        ///      }
        ///    ]
        /// }
        /// ```
        #[prost(message, tag = "4")]
        ButtonList(super::ButtonList),
        /// Displays a text box that users can type into.
        ///
        /// For example, the following JSON creates a text input for an email
        /// address:
        ///
        /// ```
        /// "textInput": {
        ///    "name": "mailing_address",
        ///    "label": "Mailing Address"
        /// }
        /// ```
        ///
        /// As another example, the following JSON creates a text input for a
        /// programming language with static suggestions:
        /// ```
        /// "textInput": {
        ///    "name": "preferred_programing_language",
        ///    "label": "Preferred Language",
        ///    "initialSuggestions": {
        ///      "items": [
        ///        {
        ///          "text": "C++"
        ///        },
        ///        {
        ///          "text": "Java"
        ///        },
        ///        {
        ///          "text": "JavaScript"
        ///        },
        ///        {
        ///          "text": "Python"
        ///        }
        ///      ]
        ///    }
        /// }
        /// ```
        #[prost(message, tag = "5")]
        TextInput(super::TextInput),
        /// Displays a selection control that lets users select items. Selection
        /// controls can be checkboxes, radio buttons, switches, or dropdown menus.
        ///
        /// For example, the following JSON creates a dropdown menu that lets users
        /// choose a size:
        ///
        /// ```
        /// "selectionInput": {
        ///    "name": "size",
        ///    "label": "Size"
        ///    "type": "DROPDOWN",
        ///    "items": [
        ///      {
        ///        "text": "S",
        ///        "value": "small",
        ///        "selected": false
        ///      },
        ///      {
        ///        "text": "M",
        ///        "value": "medium",
        ///        "selected": true
        ///      },
        ///      {
        ///        "text": "L",
        ///        "value": "large",
        ///        "selected": false
        ///      },
        ///      {
        ///        "text": "XL",
        ///        "value": "extra_large",
        ///        "selected": false
        ///      }
        ///    ]
        /// }
        /// ```
        #[prost(message, tag = "6")]
        SelectionInput(super::SelectionInput),
        /// Displays a widget that lets users input a date, time, or date and time.
        ///
        /// For example, the following JSON creates a date time picker to schedule an
        /// appointment:
        ///
        ///
        /// ```
        /// "dateTimePicker": {
        ///    "name": "appointment_time",
        ///    "label": "Book your appointment at:",
        ///    "type": "DATE_AND_TIME",
        ///    "valueMsEpoch": "796435200000"
        /// }
        /// ```
        #[prost(message, tag = "7")]
        DateTimePicker(super::DateTimePicker),
        /// Displays a horizontal line divider between widgets.
        ///
        /// For example, the following JSON creates a divider:
        /// ```
        /// "divider": {
        /// }
        /// ```
        #[prost(message, tag = "9")]
        Divider(super::Divider),
        /// Displays a grid with a collection of items.
        ///
        /// A grid supports any number of columns and items. The number of rows is
        /// determined by the upper bounds of the number items divided by the number
        /// of columns. A grid with 10 items and 2 columns has 5 rows. A grid with 11
        /// items and 2 columns has 6 rows.
        ///
        /// [Google Workspace Add-ons and
        /// Chat apps](<https://developers.google.com/workspace/extend>):
        ///
        /// For example, the following JSON creates a 2 column grid with a single
        /// item:
        ///
        /// ```
        /// "grid": {
        ///    "title": "A fine collection of items",
        ///    "columnCount": 2,
        ///    "borderStyle": {
        ///      "type": "STROKE",
        ///      "cornerRadius": 4
        ///    },
        ///    "items": [
        ///      {
        ///        "image": {
        ///          "imageUri": "<https://www.example.com/image.png",>
        ///          "cropStyle": {
        ///            "type": "SQUARE"
        ///          },
        ///          "borderStyle": {
        ///            "type": "STROKE"
        ///          }
        ///        },
        ///        "title": "An item",
        ///        "textAlignment": "CENTER"
        ///      }
        ///    ],
        ///    "onClick": {
        ///      "openLink": {
        ///        "url": "<https://www.example.com">
        ///      }
        ///    }
        /// }
        /// ```
        #[prost(message, tag = "10")]
        Grid(super::Grid),
        /// Displays up to 2 columns.
        ///
        /// To include more than 2 columns, or to use rows, use the `Grid` widget.
        ///
        /// For example, the following JSON creates 2 columns that each contain
        /// text paragraphs:
        ///
        /// ```
        /// "columns": {
        ///    "columnItems": [
        ///      {
        ///        "horizontalSizeStyle": "FILL_AVAILABLE_SPACE",
        ///        "horizontalAlignment": "CENTER",
        ///        "verticalAlignment": "CENTER",
        ///        "widgets": [
        ///          {
        ///            "textParagraph": {
        ///              "text": "First column text paragraph"
        ///            }
        ///          }
        ///        ]
        ///      },
        ///      {
        ///        "horizontalSizeStyle": "FILL_AVAILABLE_SPACE",
        ///        "horizontalAlignment": "CENTER",
        ///        "verticalAlignment": "CENTER",
        ///        "widgets": [
        ///          {
        ///            "textParagraph": {
        ///              "text": "Second column text paragraph"
        ///            }
        ///          }
        ///        ]
        ///      }
        ///    ]
        /// }
        /// ```
        #[prost(message, tag = "11")]
        Columns(super::Columns),
    }
}
/// A paragraph of text that supports formatting. For an example in
/// Google Chat apps, see [Text
/// paragraph](<https://developers.google.com/chat/ui/widgets/text-paragraph>).
/// For more information
/// about formatting text, see
/// [Formatting text in Google Chat
/// apps](<https://developers.google.com/chat/format-messages#card-formatting>)
/// and
/// [Formatting
/// text in Google Workspace
/// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
///
/// [Google Workspace Add-ons and
/// Chat apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextParagraph {
    /// The text that's shown in the widget.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// An image that is specified by a URL and can have an `onClick` action. For an
/// example, see [Image](<https://developers.google.com/chat/ui/widgets/image>).
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// The HTTPS URL that hosts the image.
    ///
    /// For example:
    ///
    /// ```
    /// <https://developers.google.com/chat/images/quickstart-app-avatar.png>
    /// ```
    #[prost(string, tag = "1")]
    pub image_url: ::prost::alloc::string::String,
    /// When a user clicks the image, the click triggers this action.
    #[prost(message, optional, tag = "2")]
    pub on_click: ::core::option::Option<OnClick>,
    /// The alternative text of this image that's used for accessibility.
    #[prost(string, tag = "3")]
    pub alt_text: ::prost::alloc::string::String,
}
/// Displays a divider between widgets as a horizontal line. For an example in
/// Google Chat apps, see
/// [Divider](<https://developers.google.com/chat/ui/widgets/divider>).
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
///
/// For example, the following JSON creates a divider:
///
/// ```
/// "divider": {}
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Divider {}
/// A widget that displays text with optional decorations such as a label above
/// or below the text, an icon in front of the text, a selection widget, or a
/// button after the text. For an example in
/// Google Chat apps, see [Decorated
/// text](<https://developers.google.com/chat/ui/widgets/decorated-text>).
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecoratedText {
    /// Deprecated in favor of `startIcon`.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub icon: ::core::option::Option<Icon>,
    /// The icon displayed in front of the text.
    #[prost(message, optional, tag = "12")]
    pub start_icon: ::core::option::Option<Icon>,
    /// The text that appears above `text`. Always truncates.
    #[prost(string, tag = "3")]
    pub top_label: ::prost::alloc::string::String,
    /// Required. The primary text.
    ///
    /// Supports simple formatting. For more information
    /// about formatting text, see
    /// [Formatting text in Google Chat
    /// apps](<https://developers.google.com/chat/format-messages#card-formatting>)
    /// and
    /// [Formatting
    /// text in Google Workspace
    /// Add-ons](<https://developers.google.com/apps-script/add-ons/concepts/widgets#text_formatting>).
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// The wrap text setting. If `true`, the text wraps and displays on
    /// multiple lines. Otherwise, the text is truncated.
    ///
    /// Only applies to `text`, not `topLabel` and `bottomLabel`.
    #[prost(bool, tag = "5")]
    pub wrap_text: bool,
    /// The text that appears below `text`. Always wraps.
    #[prost(string, tag = "6")]
    pub bottom_label: ::prost::alloc::string::String,
    /// This action is triggered when users click `topLabel` or `bottomLabel`.
    #[prost(message, optional, tag = "7")]
    pub on_click: ::core::option::Option<OnClick>,
    /// A button, switch, checkbox, or image that appears to the right-hand side
    /// of text in the `decoratedText` widget.
    #[prost(oneof = "decorated_text::Control", tags = "8, 9, 11")]
    pub control: ::core::option::Option<decorated_text::Control>,
}
/// Nested message and enum types in `DecoratedText`.
pub mod decorated_text {
    /// Either a toggle-style switch or a checkbox inside a `decoratedText` widget.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    ///
    /// Only supported in the `decoratedText` widget.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SwitchControl {
        /// The name by which the switch widget is identified in a form input event.
        ///
        /// For details about working with form inputs, see [Receive form
        /// data](<https://developers.google.com/chat/ui/read-form-data>).
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The value entered by a user, returned as part of a form input event.
        ///
        /// For details about working with form inputs, see [Receive form
        /// data](<https://developers.google.com/chat/ui/read-form-data>).
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        /// When `true`, the switch is selected.
        #[prost(bool, tag = "3")]
        pub selected: bool,
        /// The action to perform when the switch state is changed, such as what
        ///   function to run.
        #[prost(message, optional, tag = "4")]
        pub on_change_action: ::core::option::Option<super::Action>,
        /// How the switch appears in the user interface.
        ///
        /// [Google Workspace Add-ons
        /// and Chat apps](<https://developers.google.com/workspace/extend>):
        #[prost(enumeration = "switch_control::ControlType", tag = "5")]
        pub control_type: i32,
    }
    /// Nested message and enum types in `SwitchControl`.
    pub mod switch_control {
        /// How the switch appears in the user interface.
        ///
        /// [Google Workspace Add-ons
        /// and Chat apps](<https://developers.google.com/workspace/extend>):
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ControlType {
            /// A toggle-style switch.
            Switch = 0,
            /// Deprecated in favor of `CHECK_BOX`.
            Checkbox = 1,
            /// A checkbox.
            CheckBox = 2,
        }
        impl ControlType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ControlType::Switch => "SWITCH",
                    ControlType::Checkbox => "CHECKBOX",
                    ControlType::CheckBox => "CHECK_BOX",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SWITCH" => Some(Self::Switch),
                    "CHECKBOX" => Some(Self::Checkbox),
                    "CHECK_BOX" => Some(Self::CheckBox),
                    _ => None,
                }
            }
        }
    }
    /// A button, switch, checkbox, or image that appears to the right-hand side
    /// of text in the `decoratedText` widget.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Control {
        /// A button that a user can click to trigger an action.
        #[prost(message, tag = "8")]
        Button(super::Button),
        /// A switch widget that a user can click to change its state and trigger an
        /// action.
        #[prost(message, tag = "9")]
        SwitchControl(SwitchControl),
        /// An icon displayed after the text.
        ///
        /// Supports
        /// [built-in](<https://developers.google.com/chat/format-messages#builtinicons>)
        /// and
        /// [custom](<https://developers.google.com/chat/format-messages#customicons>)
        /// icons.
        #[prost(message, tag = "11")]
        EndIcon(super::Icon),
    }
}
/// A field in which users can enter text. Supports suggestions and on-change
/// actions. For an example in Google Chat apps, see [Text
/// input](<https://developers.google.com/chat/ui/widgets/text-input>).
///
/// Chat apps receive and can process the value of entered text during form input
/// events. For details about working with form inputs, see [Receive form
/// data](<https://developers.google.com/chat/ui/read-form-data>).
///
/// When you need to collect undefined or abstract data from users,
/// use a text input. To collect defined or enumerated data from users, use the
/// [SelectionInput][google.apps.card.v1.SelectionInput] widget.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// The name by which the text input is identified in a form input event.
    ///
    /// For details about working with form inputs, see [Receive form
    /// data](<https://developers.google.com/chat/ui/read-form-data>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The text that appears above the text input field in the user interface.
    ///
    /// Specify text that helps the user enter the information your app needs.
    /// For example, if you are asking someone's name, but specifically need their
    /// surname, write `surname` instead of `name`.
    ///
    /// Required if `hintText` is unspecified. Otherwise, optional.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// Text that appears below the text input field meant to assist users by
    /// prompting them to enter a certain value. This text is always visible.
    ///
    /// Required if `label` is unspecified. Otherwise, optional.
    #[prost(string, tag = "3")]
    pub hint_text: ::prost::alloc::string::String,
    /// The value entered by a user, returned as part of a form input event.
    ///
    /// For details about working with form inputs, see [Receive form
    /// data](<https://developers.google.com/chat/ui/read-form-data>).
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    /// How a text input field appears in the user interface.
    /// For example, whether the field is single or multi-line.
    #[prost(enumeration = "text_input::Type", tag = "5")]
    pub r#type: i32,
    /// What to do when a change occurs in the text input field. For example, a
    /// user adding to the field or deleting text.
    ///
    /// Examples of actions to take include running a custom function or opening
    /// a [dialog](<https://developers.google.com/chat/how-tos/dialogs>)
    /// in Google Chat.
    #[prost(message, optional, tag = "6")]
    pub on_change_action: ::core::option::Option<Action>,
    /// Suggested values that users can enter. These values appear when users click
    /// inside the text input field. As users type, the suggested values
    /// dynamically filter to match what the users have typed.
    ///
    /// For example, a text input field for programming language might suggest
    /// Java, JavaScript, Python, and C++. When users start typing `Jav`, the list
    /// of suggestions filters to show just `Java` and `JavaScript`.
    ///
    /// Suggested values help guide users to enter values that your app can make
    /// sense of. When referring to JavaScript, some users might enter `javascript`
    /// and others `java script`. Suggesting `JavaScript` can standardize how users
    /// interact with your app.
    ///
    /// When specified, `TextInput.type` is always `SINGLE_LINE`, even if it's set
    /// to `MULTIPLE_LINE`.
    ///
    /// [Google Workspace
    /// Add-ons and Chat apps](<https://developers.google.com/workspace/extend>):
    #[prost(message, optional, tag = "7")]
    pub initial_suggestions: ::core::option::Option<Suggestions>,
    /// Optional. Specify what action to take when the text input field provides
    /// suggestions to users who interact with it.
    ///
    /// If unspecified, the suggestions are set by `initialSuggestions` and
    /// are processed by the client.
    ///
    /// If specified, the app takes the action specified here, such as running
    /// a custom function.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[prost(message, optional, tag = "8")]
    pub auto_complete_action: ::core::option::Option<Action>,
    /// Text that appears in the text input field when the field is empty.
    /// Use this text to prompt users to enter a value. For example, `Enter a
    /// number from 0 to 100`.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[prost(string, tag = "12")]
    pub placeholder_text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TextInput`.
pub mod text_input {
    /// How a text input field appears in the user interface. For example,
    /// whether it's a single line input field, or a multi-line input. If
    /// `initialSuggestions` is specified, `type` is always `SINGLE_LINE`,
    /// even if it's set to `MULTIPLE_LINE`.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// The text input field has a fixed height of one line.
        SingleLine = 0,
        /// The text input field has a fixed height of multiple lines.
        MultipleLine = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::SingleLine => "SINGLE_LINE",
                Type::MultipleLine => "MULTIPLE_LINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SINGLE_LINE" => Some(Self::SingleLine),
                "MULTIPLE_LINE" => Some(Self::MultipleLine),
                _ => None,
            }
        }
    }
}
/// Suggested values that users can enter. These values appear when users click
/// inside the text input field. As users type, the suggested values
/// dynamically filter to match what the users have typed.
///
/// For example, a text input field for programming language might suggest
/// Java, JavaScript, Python, and C++. When users start typing `Jav`, the list
/// of suggestions filters to show `Java` and `JavaScript`.
///
/// Suggested values help guide users to enter values that your app can make
/// sense of. When referring to JavaScript, some users might enter `javascript`
/// and others `java script`. Suggesting `JavaScript` can standardize how users
/// interact with your app.
///
/// When specified, `TextInput.type` is always `SINGLE_LINE`, even if it's set
/// to `MULTIPLE_LINE`.
///
/// [Google Workspace
/// Add-ons and Chat apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Suggestions {
    /// A list of suggestions used for autocomplete recommendations in text input
    /// fields.
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<suggestions::SuggestionItem>,
}
/// Nested message and enum types in `Suggestions`.
pub mod suggestions {
    /// One suggested value that users can enter in a text input field.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuggestionItem {
        #[prost(oneof = "suggestion_item::Content", tags = "1")]
        pub content: ::core::option::Option<suggestion_item::Content>,
    }
    /// Nested message and enum types in `SuggestionItem`.
    pub mod suggestion_item {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Content {
            /// The value of a suggested input to a text input field. This is
            /// equivalent to what users enter themselves.
            #[prost(string, tag = "1")]
            Text(::prost::alloc::string::String),
        }
    }
}
/// A list of buttons layed out horizontally. For an example in
/// Google Chat apps, see
/// [Button list](<https://developers.google.com/chat/ui/widgets/button-list>).
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonList {
    /// An array of buttons.
    #[prost(message, repeated, tag = "1")]
    pub buttons: ::prost::alloc::vec::Vec<Button>,
}
/// A widget that creates one or more UI items that users can select.
/// For example, a dropdown menu or checkboxes. You can use this widget to
/// collect data that can be predicted or enumerated. For an example in Google
/// Chat apps, see [Selection
/// input](<https://developers.google.com/chat/ui/widgets/selection-input>).
///
/// Chat apps can process the value of items that users select or input. For
/// details about working with form inputs, see [Receive form
/// data](<https://developers.google.com/chat/ui/read-form-data>).
///
/// To collect undefined or abstract data from users, use
/// the [TextInput][google.apps.card.v1.TextInput] widget.
///
/// [Google Workspace Add-ons
/// and Chat apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectionInput {
    /// The name that identifies the selection input in a form input event.
    ///
    /// For details about working with form inputs, see [Receive form
    /// data](<https://developers.google.com/chat/ui/read-form-data>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The text that appears above the selection input field in the user
    /// interface.
    ///
    /// Specify text that helps the user enter the information your app needs.
    /// For example, if users are selecting the urgency of a work ticket from a
    /// drop-down menu, the label might be "Urgency" or "Select urgency".
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// The type of items that are displayed to users in a `SelectionInput` widget.
    /// Selection types support different types of interactions. For example, users
    /// can select one or more checkboxes, but they can only select one value from
    /// a dropdown menu.
    #[prost(enumeration = "selection_input::SelectionType", tag = "3")]
    pub r#type: i32,
    /// An array of selectable items. For example, an array of radio buttons or
    /// checkboxes. Supports up to 100 items.
    #[prost(message, repeated, tag = "4")]
    pub items: ::prost::alloc::vec::Vec<selection_input::SelectionItem>,
    /// If specified, the form is submitted when the selection changes. If not
    /// specified, you must specify a separate button that submits the form.
    ///
    /// For details about working with form inputs, see [Receive form
    /// data](<https://developers.google.com/chat/ui/read-form-data>).
    #[prost(message, optional, tag = "5")]
    pub on_change_action: ::core::option::Option<Action>,
    /// For multiselect menus, the maximum number of items that a user can select.
    /// Minimum value is 1 item. If unspecified, defaults to 3 items.
    #[prost(int32, tag = "6")]
    pub multi_select_max_selected_items: i32,
    /// For multiselect menus, the number of text characters that a user inputs
    /// before the Chat app queries autocomplete and displays suggested items
    /// in the menu.
    ///
    /// If unspecified, defaults to 0 characters for static data sources and 3
    /// characters for external data sources.
    #[prost(int32, tag = "7")]
    pub multi_select_min_query_length: i32,
    /// For a multiselect menu, the data source that populates
    /// selection items.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[prost(oneof = "selection_input::MultiSelectDataSource", tags = "8, 9")]
    pub multi_select_data_source: ::core::option::Option<
        selection_input::MultiSelectDataSource,
    >,
}
/// Nested message and enum types in `SelectionInput`.
pub mod selection_input {
    /// An item that users can select in a selection input, such as a checkbox
    /// or switch.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelectionItem {
        /// The text that identifies or describes the item to users.
        #[prost(string, tag = "1")]
        pub text: ::prost::alloc::string::String,
        /// The value associated with this item. The client should use this as a form
        /// input value.
        ///
        /// For details about working with form inputs, see [Receive form
        /// data](<https://developers.google.com/chat/ui/read-form-data>).
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        /// Whether the item is selected by default. If the selection input only
        /// accepts one value (such as for radio buttons or a dropdown menu), only
        /// set this field for one item.
        #[prost(bool, tag = "3")]
        pub selected: bool,
        /// For multiselect menus, the URL for the icon displayed next to
        /// the item's `text` field. Supports PNG and JPEG files. Must be an `HTTPS`
        /// URL. For example,
        /// `<https://developers.google.com/chat/images/quickstart-app-avatar.png`.>
        #[prost(string, tag = "4")]
        pub start_icon_uri: ::prost::alloc::string::String,
        /// For multiselect menus, a text description or label that's
        /// displayed below the item's `text` field.
        #[prost(string, tag = "5")]
        pub bottom_text: ::prost::alloc::string::String,
    }
    /// For a
    /// [`SelectionInput`][google.apps.card.v1.SelectionInput] widget that uses a
    /// multiselect menu, a data source from Google Workspace. Used to populate
    /// items in a multiselect menu.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlatformDataSource {
        /// The data source.
        #[prost(oneof = "platform_data_source::DataSource", tags = "1")]
        pub data_source: ::core::option::Option<platform_data_source::DataSource>,
    }
    /// Nested message and enum types in `PlatformDataSource`.
    pub mod platform_data_source {
        /// A data source shared by all [Google Workspace
        /// applications]
        /// (<https://developers.google.com/chat/api/reference/rest/v1/HostApp>).
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum CommonDataSource {
            /// Default value. Don't use.
            Unknown = 0,
            /// Google Workspace users. The user can only view and select users from
            /// their Google Workspace organization.
            User = 1,
        }
        impl CommonDataSource {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    CommonDataSource::Unknown => "UNKNOWN",
                    CommonDataSource::User => "USER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "USER" => Some(Self::User),
                    _ => None,
                }
            }
        }
        /// The data source.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DataSource {
            /// A data source shared by all Google Workspace applications, such as
            /// users in a Google Workspace organization.
            #[prost(enumeration = "CommonDataSource", tag = "1")]
            CommonDataSource(i32),
        }
    }
    /// The format for the items that users can select. Different options support
    /// different types of interactions. For example, users can select multiple
    /// checkboxes, but can only select one item from a dropdown menu.
    ///
    /// Each selection input supports one type of selection. Mixing checkboxes
    /// and switches, for example, isn't supported.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SelectionType {
        /// A set of checkboxes. Users can select one or more checkboxes.
        CheckBox = 0,
        /// A set of radio buttons. Users can select one radio button.
        RadioButton = 1,
        /// A set of switches. Users can turn on one or more switches.
        Switch = 2,
        /// A dropdown menu. Users can select one item from the menu.
        Dropdown = 3,
        /// A multiselect menu for static or dynamic data. From the menu bar,
        /// users select one or more items. Users can also input values to populate
        /// dynamic data. For example, users can start typing the name of a Google
        /// Chat space and the widget autosuggests the space.
        ///
        /// To populate items for a multiselect menu, you can use one of the
        /// following types of data sources:
        ///
        ///   * Static data: Items are specified as `SelectionItem` objects in the
        ///     widget. Up to 100 items.
        ///   * Google Workspace data: Items are populated using data from Google
        ///     Workspace, such as Google Workspace users or Google Chat spaces.
        ///   * External data: Items are populated from an external data
        ///     source outside of Google Workspace.
        ///
        /// For examples of how to implement multiselect menus, see the
        /// [`SelectionInput` widget
        /// page](<https://developers.google.com/chat/ui/widgets/selection-input#multiselect-menu>).
        ///
        /// [Google Workspace Add-ons and Chat
        /// apps](<https://developers.google.com/workspace/extend>):
        /// multiselect for Google Workspace Add-ons are in
        /// [Developer Preview](<https://developers.google.com/workspace/preview>).
        MultiSelect = 4,
    }
    impl SelectionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SelectionType::CheckBox => "CHECK_BOX",
                SelectionType::RadioButton => "RADIO_BUTTON",
                SelectionType::Switch => "SWITCH",
                SelectionType::Dropdown => "DROPDOWN",
                SelectionType::MultiSelect => "MULTI_SELECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CHECK_BOX" => Some(Self::CheckBox),
                "RADIO_BUTTON" => Some(Self::RadioButton),
                "SWITCH" => Some(Self::Switch),
                "DROPDOWN" => Some(Self::Dropdown),
                "MULTI_SELECT" => Some(Self::MultiSelect),
                _ => None,
            }
        }
    }
    /// For a multiselect menu, the data source that populates
    /// selection items.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MultiSelectDataSource {
        /// An external data source, such as a relational data base.
        #[prost(message, tag = "8")]
        ExternalDataSource(super::Action),
        /// A data source from Google Workspace.
        #[prost(message, tag = "9")]
        PlatformDataSource(PlatformDataSource),
    }
}
/// Lets users input a date, a time, or both a date and a time. For an example in
/// Google Chat apps, see [Date time
/// picker](<https://developers.google.com/chat/ui/widgets/date-time-picker>).
///
/// Users can input text or use the picker to select dates and times. If users
/// input an invalid date or time, the picker shows an error that prompts users
/// to input the information correctly.
///
/// [Google Workspace
/// Add-ons and Chat apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimePicker {
    /// The name by which the `DateTimePicker` is identified in a form input event.
    ///
    /// For details about working with form inputs, see [Receive form
    /// data](<https://developers.google.com/chat/ui/read-form-data>).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The text that prompts users to input a date, a time, or a date and time.
    /// For example, if users are scheduling an appointment, use a label such as
    /// `Appointment date` or `Appointment date and time`.
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// Whether the widget supports inputting a date, a time, or the date and time.
    #[prost(enumeration = "date_time_picker::DateTimePickerType", tag = "3")]
    pub r#type: i32,
    /// The default value displayed in the widget, in milliseconds since [Unix
    /// epoch time](<https://en.wikipedia.org/wiki/Unix_time>).
    ///
    /// Specify the value based on the type of picker (`DateTimePickerType`):
    ///
    /// * `DATE_AND_TIME`: a calendar date and time in UTC. For example, to
    ///    represent January 1, 2023 at 12:00 PM UTC, use `1672574400000`.
    /// * `DATE_ONLY`: a calendar date at 00:00:00 UTC. For example, to represent
    ///    January 1, 2023, use `1672531200000`.
    /// * `TIME_ONLY`: a time in UTC. For example, to represent 12:00 PM, use
    ///    `43200000` (or `12 * 60 * 60 * 1000`).
    #[prost(int64, tag = "4")]
    pub value_ms_epoch: i64,
    /// The number representing the time zone offset from UTC, in minutes.
    /// If set, the `value_ms_epoch` is displayed in the specified time zone.
    /// If unset, the value defaults to the user's time zone setting.
    #[prost(int32, tag = "5")]
    pub timezone_offset_date: i32,
    /// Triggered when the user clicks **Save** or **Clear** from the
    /// `DateTimePicker` interface.
    #[prost(message, optional, tag = "6")]
    pub on_change_action: ::core::option::Option<Action>,
}
/// Nested message and enum types in `DateTimePicker`.
pub mod date_time_picker {
    /// The format for the date and time in the `DateTimePicker` widget.
    /// Determines whether users can input a date, a time, or both a date and time.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DateTimePickerType {
        /// Users input a date and time.
        DateAndTime = 0,
        /// Users input a date.
        DateOnly = 1,
        /// Users input a time.
        TimeOnly = 2,
    }
    impl DateTimePickerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DateTimePickerType::DateAndTime => "DATE_AND_TIME",
                DateTimePickerType::DateOnly => "DATE_ONLY",
                DateTimePickerType::TimeOnly => "TIME_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATE_AND_TIME" => Some(Self::DateAndTime),
                "DATE_ONLY" => Some(Self::DateOnly),
                "TIME_ONLY" => Some(Self::TimeOnly),
                _ => None,
            }
        }
    }
}
/// A text, icon, or text and icon button that users can click. For an example in
/// Google Chat apps, see
/// [Button list](<https://developers.google.com/chat/ui/widgets/button-list>).
///
/// To make an image a clickable button, specify an
/// [`Image`][google.apps.card.v1.Image] (not an
/// [`ImageComponent`][google.apps.card.v1.ImageComponent]) and set an
/// `onClick` action.
///
/// [Google Workspace
/// Add-ons and Chat apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    /// The text displayed inside the button.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The icon image. If both `icon` and `text` are set, then the icon appears
    /// before the text.
    #[prost(message, optional, tag = "2")]
    pub icon: ::core::option::Option<Icon>,
    /// If set, the button is filled with a solid background color and the font
    /// color changes to maintain contrast with the background color. For example,
    /// setting a blue background likely results in white text.
    ///
    /// If unset, the image background is white and the font color is blue.
    ///
    /// For red, green, and blue, the value of each field is a `float` number that
    /// you can express in either of two ways: as a number between 0 and 255
    /// divided by 255 (153/255), or as a value between 0 and 1 (0.6). 0 represents
    /// the absence of a color and 1 or 255/255 represent the full presence of that
    /// color on the RGB scale.
    ///
    /// Optionally set `alpha`, which sets a level of transparency using this
    /// equation:
    ///
    /// ```
    /// pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
    /// ```
    ///
    /// For `alpha`, a value of `1` corresponds with a solid color, and a value of
    /// `0` corresponds with a completely transparent color.
    ///
    /// For example, the following color represents a half transparent red:
    ///
    /// ```
    /// "color": {
    ///     "red": 1,
    ///     "green": 0,
    ///     "blue": 0,
    ///     "alpha": 0.5
    /// }
    /// ```
    #[prost(message, optional, tag = "3")]
    pub color: ::core::option::Option<super::super::super::r#type::Color>,
    /// Required. The action to perform when a user clicks the button, such as
    /// opening a hyperlink or running a custom function.
    #[prost(message, optional, boxed, tag = "4")]
    pub on_click: ::core::option::Option<::prost::alloc::boxed::Box<OnClick>>,
    /// If `true`, the button is displayed in an inactive state and doesn't respond
    /// to user actions.
    #[prost(bool, tag = "5")]
    pub disabled: bool,
    /// The alternative text that's used for accessibility.
    ///
    /// Set descriptive text that lets users know what the button does. For
    /// example, if a button opens a hyperlink, you might write: "Opens a new
    /// browser tab and navigates to the Google Chat developer documentation at
    /// <https://developers.google.com/chat".>
    #[prost(string, tag = "6")]
    pub alt_text: ::prost::alloc::string::String,
}
/// An icon displayed in a widget on a card. For an example in Google Chat apps,
/// see [Icon](<https://developers.google.com/chat/ui/widgets/icon>).
///
/// Supports
/// [built-in](<https://developers.google.com/chat/format-messages#builtinicons>)
/// and
/// [custom](<https://developers.google.com/chat/format-messages#customicons>)
/// icons.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Icon {
    /// Optional. A description of the icon used for accessibility.
    /// If unspecified, the default value `Button` is provided. As a best practice,
    /// you should set a helpful description for what the icon displays, and if
    /// applicable, what it does. For example, `A user's account portrait`, or
    /// `Opens a new browser tab and navigates to the Google Chat developer
    /// documentation at <https://developers.google.com/chat`.>
    ///
    /// If the icon is set in a [`Button`][google.apps.card.v1.Button], the
    /// `altText` appears as helper text when the user hovers over the button.
    /// However, if the button also sets `text`, the icon's `altText` is ignored.
    #[prost(string, tag = "3")]
    pub alt_text: ::prost::alloc::string::String,
    /// The crop style applied to the image. In some cases, applying a
    /// `CIRCLE` crop causes the image to be drawn larger than a built-in
    /// icon.
    #[prost(enumeration = "widget::ImageType", tag = "4")]
    pub image_type: i32,
    /// The icon displayed in the widget on the card.
    #[prost(oneof = "icon::Icons", tags = "1, 2")]
    pub icons: ::core::option::Option<icon::Icons>,
}
/// Nested message and enum types in `Icon`.
pub mod icon {
    /// The icon displayed in the widget on the card.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Icons {
        /// Display one of the built-in icons provided by Google Workspace.
        ///
        /// For example, to display an airplane icon, specify `AIRPLANE`.
        /// For a bus, specify `BUS`.
        ///
        /// For a full list of supported icons, see [built-in
        /// icons](<https://developers.google.com/chat/format-messages#builtinicons>).
        #[prost(string, tag = "1")]
        KnownIcon(::prost::alloc::string::String),
        /// Display a custom icon hosted at an HTTPS URL.
        ///
        /// For example:
        ///
        /// ```
        /// "iconUrl":
        /// "<https://developers.google.com/chat/images/quickstart-app-avatar.png">
        /// ```
        ///
        /// Supported file types include `.png` and `.jpg`.
        #[prost(string, tag = "2")]
        IconUrl(::prost::alloc::string::String),
    }
}
/// Represents the crop style applied to an image.
///
/// [Google Workspace Add-ons and
/// Chat apps](<https://developers.google.com/workspace/extend>):
///
/// For example, here's how to apply a 16:9 aspect ratio:
///
/// ```
/// cropStyle {
///   "type": "RECTANGLE_CUSTOM",
///   "aspectRatio": 16/9
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageCropStyle {
    /// The crop type.
    #[prost(enumeration = "image_crop_style::ImageCropType", tag = "1")]
    pub r#type: i32,
    /// The aspect ratio to use if the crop type is `RECTANGLE_CUSTOM`.
    ///
    /// For example, here's how to apply a 16:9 aspect ratio:
    ///
    /// ```
    /// cropStyle {
    ///   "type": "RECTANGLE_CUSTOM",
    ///   "aspectRatio": 16/9
    /// }
    /// ```
    #[prost(double, tag = "2")]
    pub aspect_ratio: f64,
}
/// Nested message and enum types in `ImageCropStyle`.
pub mod image_crop_style {
    /// Represents the crop style applied to an image.
    ///
    /// [Google Workspace Add-ons
    /// and Chat apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ImageCropType {
        /// Don't use. Unspecified.
        Unspecified = 0,
        /// Default value. Applies a square crop.
        Square = 1,
        /// Applies a circular crop.
        Circle = 2,
        /// Applies a rectangular crop with a custom aspect ratio. Set the custom
        /// aspect ratio with `aspectRatio`.
        RectangleCustom = 3,
        /// Applies a rectangular crop with a 4:3 aspect ratio.
        Rectangle43 = 4,
    }
    impl ImageCropType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImageCropType::Unspecified => "IMAGE_CROP_TYPE_UNSPECIFIED",
                ImageCropType::Square => "SQUARE",
                ImageCropType::Circle => "CIRCLE",
                ImageCropType::RectangleCustom => "RECTANGLE_CUSTOM",
                ImageCropType::Rectangle43 => "RECTANGLE_4_3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMAGE_CROP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SQUARE" => Some(Self::Square),
                "CIRCLE" => Some(Self::Circle),
                "RECTANGLE_CUSTOM" => Some(Self::RectangleCustom),
                "RECTANGLE_4_3" => Some(Self::Rectangle43),
                _ => None,
            }
        }
    }
}
/// The style options for the border of a card or widget, including the border
/// type and color.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorderStyle {
    /// The border type.
    #[prost(enumeration = "border_style::BorderType", tag = "1")]
    pub r#type: i32,
    /// The colors to use when the type is `BORDER_TYPE_STROKE`.
    #[prost(message, optional, tag = "2")]
    pub stroke_color: ::core::option::Option<super::super::super::r#type::Color>,
    /// The corner radius for the border.
    #[prost(int32, tag = "3")]
    pub corner_radius: i32,
}
/// Nested message and enum types in `BorderStyle`.
pub mod border_style {
    /// Represents the border types applied to widgets.
    ///
    /// [Google Workspace Add-ons
    /// and Chat apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BorderType {
        /// Don't use. Unspecified.
        Unspecified = 0,
        /// Default value. No border.
        NoBorder = 1,
        /// Outline.
        Stroke = 2,
    }
    impl BorderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BorderType::Unspecified => "BORDER_TYPE_UNSPECIFIED",
                BorderType::NoBorder => "NO_BORDER",
                BorderType::Stroke => "STROKE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NO_BORDER" => Some(Self::NoBorder),
                "STROKE" => Some(Self::Stroke),
                _ => None,
            }
        }
    }
}
/// Represents an image.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageComponent {
    /// The image URL.
    #[prost(string, tag = "1")]
    pub image_uri: ::prost::alloc::string::String,
    /// The accessibility label for the image.
    #[prost(string, tag = "2")]
    pub alt_text: ::prost::alloc::string::String,
    /// The crop style to apply to the image.
    #[prost(message, optional, tag = "3")]
    pub crop_style: ::core::option::Option<ImageCropStyle>,
    /// The border style to apply to the image.
    #[prost(message, optional, tag = "4")]
    pub border_style: ::core::option::Option<BorderStyle>,
}
/// Displays a grid with a collection of items. Items can only include text or
/// images. For responsive columns, or to include more than text or images, use
/// [`Columns`][google.apps.card.v1.Columns]. For an example in Google Chat apps,
/// see [Grid](<https://developers.google.com/chat/ui/widgets/grid>).
///
/// A grid supports any number of columns and items. The number of rows is
/// determined by items divided by columns. A grid with
/// 10 items and 2 columns has 5 rows. A grid with 11 items and 2 columns
/// has 6 rows.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
///
/// For example, the following JSON creates a 2 column grid with a single
/// item:
///
/// ```
/// "grid": {
///    "title": "A fine collection of items",
///    "columnCount": 2,
///    "borderStyle": {
///      "type": "STROKE",
///      "cornerRadius": 4
///    },
///    "items": [
///      {
///        "image": {
///          "imageUri": "<https://www.example.com/image.png",>
///          "cropStyle": {
///            "type": "SQUARE"
///          },
///          "borderStyle": {
///            "type": "STROKE"
///          }
///        },
///        "title": "An item",
///        "textAlignment": "CENTER"
///      }
///    ],
///    "onClick": {
///      "openLink": {
///        "url": "<https://www.example.com">
///      }
///    }
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grid {
    /// The text that displays in the grid header.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// The items to display in the grid.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<grid::GridItem>,
    /// The border style to apply to each grid item.
    #[prost(message, optional, tag = "3")]
    pub border_style: ::core::option::Option<BorderStyle>,
    /// The number of columns to display in the grid. A default value
    /// is used if this field isn't specified, and that default value is
    /// different depending on where the grid is shown (dialog versus companion).
    #[prost(int32, tag = "4")]
    pub column_count: i32,
    /// This callback is reused by each individual grid item, but with the
    /// item's identifier and index in the items list added to the callback's
    /// parameters.
    #[prost(message, optional, tag = "5")]
    pub on_click: ::core::option::Option<OnClick>,
}
/// Nested message and enum types in `Grid`.
pub mod grid {
    /// Represents an item in a grid layout. Items can contain text, an image, or
    /// both text and an image.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GridItem {
        /// A user-specified identifier for this grid item. This identifier is
        /// returned in the parent grid's `onClick` callback parameters.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The image that displays in the grid item.
        #[prost(message, optional, tag = "2")]
        pub image: ::core::option::Option<super::ImageComponent>,
        /// The grid item's title.
        #[prost(string, tag = "3")]
        pub title: ::prost::alloc::string::String,
        /// The grid item's subtitle.
        #[prost(string, tag = "4")]
        pub subtitle: ::prost::alloc::string::String,
        /// The layout to use for the grid item.
        #[prost(enumeration = "grid_item::GridItemLayout", tag = "9")]
        pub layout: i32,
    }
    /// Nested message and enum types in `GridItem`.
    pub mod grid_item {
        /// Represents the various layout options available for a grid item.
        ///
        /// [Google Workspace Add-ons and Chat
        /// apps](<https://developers.google.com/workspace/extend>):
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum GridItemLayout {
            /// Don't use. Unspecified.
            Unspecified = 0,
            /// The title and subtitle are shown below the grid item's image.
            TextBelow = 1,
            /// The title and subtitle are shown above the grid item's image.
            TextAbove = 2,
        }
        impl GridItemLayout {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    GridItemLayout::Unspecified => "GRID_ITEM_LAYOUT_UNSPECIFIED",
                    GridItemLayout::TextBelow => "TEXT_BELOW",
                    GridItemLayout::TextAbove => "TEXT_ABOVE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "GRID_ITEM_LAYOUT_UNSPECIFIED" => Some(Self::Unspecified),
                    "TEXT_BELOW" => Some(Self::TextBelow),
                    "TEXT_ABOVE" => Some(Self::TextAbove),
                    _ => None,
                }
            }
        }
    }
}
/// The `Columns` widget displays up to 2 columns in a card or dialog. You can
/// add widgets to each column; the widgets appear in the order that they are
/// specified. For an example in Google Chat apps, see
/// [Columns](<https://developers.google.com/chat/ui/widgets/columns>).
///
/// The height of each column is determined by the taller column. For example, if
/// the first column is taller than the second column, both columns have the
/// height of the first column. Because each column can contain a different
/// number of widgets, you can't define rows or align widgets between the
/// columns.
///
/// Columns are displayed side-by-side. You can customize the width of each
/// column using the `HorizontalSizeStyle` field. If the user's
/// screen width is too narrow, the second column wraps below the first:
///
/// * On web, the second column wraps if the screen width is less than or equal
///    to 480 pixels.
/// * On iOS devices, the second column wraps if the screen width is
///    less than or equal to 300 pt.
/// * On Android devices, the second column wraps if the screen width is
///    less than or equal to 320 dp.
///
/// To include more than 2 columns, or to use rows, use the
/// [`Grid`][google.apps.card.v1.Grid] widget.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
/// Columns for Google Workspace Add-ons are in
/// [Developer Preview](<https://developers.google.com/workspace/preview>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Columns {
    /// An array of columns. You can include up to 2 columns in a card or dialog.
    #[prost(message, repeated, tag = "2")]
    pub column_items: ::prost::alloc::vec::Vec<columns::Column>,
}
/// Nested message and enum types in `Columns`.
pub mod columns {
    /// A column.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Column {
        /// Specifies how a column fills the width of the card.
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        #[prost(enumeration = "column::HorizontalSizeStyle", tag = "1")]
        pub horizontal_size_style: i32,
        /// Specifies whether widgets align to the left, right, or center of a
        /// column.
        #[prost(enumeration = "super::widget::HorizontalAlignment", tag = "2")]
        pub horizontal_alignment: i32,
        /// Specifies whether widgets align to the top, bottom, or center of a
        /// column.
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        #[prost(enumeration = "column::VerticalAlignment", tag = "3")]
        pub vertical_alignment: i32,
        /// An array of widgets included in a column. Widgets appear in the order
        /// that they are specified.
        #[prost(message, repeated, tag = "4")]
        pub widgets: ::prost::alloc::vec::Vec<column::Widgets>,
    }
    /// Nested message and enum types in `Column`.
    pub mod column {
        /// The supported widgets that you can include in a column.
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Widgets {
            #[prost(oneof = "widgets::Data", tags = "1, 2, 3, 4, 5, 6, 7")]
            pub data: ::core::option::Option<widgets::Data>,
        }
        /// Nested message and enum types in `Widgets`.
        pub mod widgets {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Data {
                /// [TextParagraph][google.apps.card.v1.TextParagraph] widget.
                #[prost(message, tag = "1")]
                TextParagraph(super::super::super::TextParagraph),
                /// [Image][google.apps.card.v1.Image] widget.
                #[prost(message, tag = "2")]
                Image(super::super::super::Image),
                /// [DecoratedText][google.apps.card.v1.DecoratedText] widget.
                #[prost(message, tag = "3")]
                DecoratedText(super::super::super::DecoratedText),
                /// [ButtonList][google.apps.card.v1.ButtonList] widget.
                #[prost(message, tag = "4")]
                ButtonList(super::super::super::ButtonList),
                /// [TextInput][google.apps.card.v1.TextInput] widget.
                #[prost(message, tag = "5")]
                TextInput(super::super::super::TextInput),
                /// [SelectionInput][google.apps.card.v1.SelectionInput] widget.
                #[prost(message, tag = "6")]
                SelectionInput(super::super::super::SelectionInput),
                /// [DateTimePicker][google.apps.card.v1.DateTimePicker] widget.
                #[prost(message, tag = "7")]
                DateTimePicker(super::super::super::DateTimePicker),
            }
        }
        /// Specifies how a column fills the width of the card. The width of each
        /// column depends on both the `HorizontalSizeStyle` and the width of the
        /// widgets within the column.
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum HorizontalSizeStyle {
            /// Don't use. Unspecified.
            Unspecified = 0,
            /// Default value. Column fills the available space, up to 70% of the
            /// card's width. If both columns are set to `FILL_AVAILABLE_SPACE`, each
            /// column fills 50% of the space.
            FillAvailableSpace = 1,
            /// Column fills the least amount of space possible and no more than 30% of
            /// the card's width.
            FillMinimumSpace = 2,
        }
        impl HorizontalSizeStyle {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    HorizontalSizeStyle::Unspecified => {
                        "HORIZONTAL_SIZE_STYLE_UNSPECIFIED"
                    }
                    HorizontalSizeStyle::FillAvailableSpace => "FILL_AVAILABLE_SPACE",
                    HorizontalSizeStyle::FillMinimumSpace => "FILL_MINIMUM_SPACE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "HORIZONTAL_SIZE_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
                    "FILL_AVAILABLE_SPACE" => Some(Self::FillAvailableSpace),
                    "FILL_MINIMUM_SPACE" => Some(Self::FillMinimumSpace),
                    _ => None,
                }
            }
        }
        /// Specifies whether widgets align to the top, bottom, or center of a
        /// column.
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum VerticalAlignment {
            /// Don't use. Unspecified.
            Unspecified = 0,
            /// Default value. Aligns widgets to the center of a column.
            Center = 1,
            /// Aligns widgets to the top of a column.
            Top = 2,
            /// Aligns widgets to the bottom of a column.
            Bottom = 3,
        }
        impl VerticalAlignment {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    VerticalAlignment::Unspecified => "VERTICAL_ALIGNMENT_UNSPECIFIED",
                    VerticalAlignment::Center => "CENTER",
                    VerticalAlignment::Top => "TOP",
                    VerticalAlignment::Bottom => "BOTTOM",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VERTICAL_ALIGNMENT_UNSPECIFIED" => Some(Self::Unspecified),
                    "CENTER" => Some(Self::Center),
                    "TOP" => Some(Self::Top),
                    "BOTTOM" => Some(Self::Bottom),
                    _ => None,
                }
            }
        }
    }
}
/// Represents how to respond when users click an interactive element on
/// a card, such as a button.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnClick {
    #[prost(oneof = "on_click::Data", tags = "1, 2, 3, 4")]
    pub data: ::core::option::Option<on_click::Data>,
}
/// Nested message and enum types in `OnClick`.
pub mod on_click {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// If specified, an action is triggered by this `onClick`.
        #[prost(message, tag = "1")]
        Action(super::Action),
        /// If specified, this `onClick` triggers an open link action.
        #[prost(message, tag = "2")]
        OpenLink(super::OpenLink),
        /// An add-on triggers this action when the action needs to open a
        /// link. This differs from the `open_link` above in that this needs to talk
        /// to server to get the link. Thus some preparation work is required for
        /// web client to do before the open link action response comes back.
        ///
        /// [Google Workspace
        /// Add-ons](<https://developers.google.com/workspace/add-ons>):
        #[prost(message, tag = "3")]
        OpenDynamicLinkAction(super::Action),
        /// A new card is pushed to the card stack after clicking if specified.
        ///
        /// [Google Workspace
        /// Add-ons](<https://developers.google.com/workspace/add-ons>):
        #[prost(message, tag = "4")]
        Card(::prost::alloc::boxed::Box<super::Card>),
    }
}
/// Represents an `onClick` event that opens a hyperlink.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenLink {
    /// The URL to open.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// How to open a link.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[prost(enumeration = "open_link::OpenAs", tag = "2")]
    pub open_as: i32,
    /// Whether the client forgets about a link after opening it, or observes it
    /// until the window closes.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[prost(enumeration = "open_link::OnClose", tag = "3")]
    pub on_close: i32,
}
/// Nested message and enum types in `OpenLink`.
pub mod open_link {
    /// When an `OnClick` action opens a link, then the client can either open it
    /// as a full-size window (if that's the frame used by the client), or an
    /// overlay (such as a pop-up). The implementation depends on the client
    /// platform capabilities, and the value selected might be ignored if the
    /// client doesn't support it. `FULL_SIZE` is supported by all clients.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OpenAs {
        /// The link opens as a full-size window (if that's the frame used by the
        /// client).
        FullSize = 0,
        /// The link opens as an overlay, such as a pop-up.
        Overlay = 1,
    }
    impl OpenAs {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OpenAs::FullSize => "FULL_SIZE",
                OpenAs::Overlay => "OVERLAY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FULL_SIZE" => Some(Self::FullSize),
                "OVERLAY" => Some(Self::Overlay),
                _ => None,
            }
        }
    }
    /// What the client does when a link opened by an `OnClick` action is closed.
    ///
    /// Implementation depends on client platform capabilities. For example, a web
    /// browser might open a link in a pop-up window with an `OnClose` handler.
    ///
    /// If both `OnOpen` and `OnClose` handlers are set, and the client platform
    /// can't support both values, `OnClose` takes precedence.
    ///
    /// [Google Workspace
    /// Add-ons](<https://developers.google.com/workspace/add-ons>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OnClose {
        /// Default value. The card doesn't reload; nothing happens.
        Nothing = 0,
        /// Reloads the card after the child window closes.
        ///
        /// If used in conjunction with
        /// [`OpenAs.OVERLAY`](<https://developers.google.com/workspace/add-ons/reference/rpc/google.apps.card.v1#openas>),
        /// the child window acts as a modal dialog and the parent card is blocked
        /// until the child window closes.
        Reload = 1,
    }
    impl OnClose {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OnClose::Nothing => "NOTHING",
                OnClose::Reload => "RELOAD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NOTHING" => Some(Self::Nothing),
                "RELOAD" => Some(Self::Reload),
                _ => None,
            }
        }
    }
}
/// An action that describes the behavior when the form is submitted.
/// For example, you can invoke an Apps Script script to handle the form.
/// If the action is triggered, the form values are sent to the server.
///
/// [Google Workspace Add-ons and Chat
/// apps](<https://developers.google.com/workspace/extend>):
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// A custom function to invoke when the containing element is
    /// clicked or othrwise activated.
    ///
    /// For example usage, see [Create interactive
    /// cards](<https://developers.google.com/chat/how-tos/cards-onclick>).
    #[prost(string, tag = "1")]
    pub function: ::prost::alloc::string::String,
    /// List of action parameters.
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<action::ActionParameter>,
    /// Specifies the loading indicator that the action displays while
    /// making the call to the action.
    #[prost(enumeration = "action::LoadIndicator", tag = "3")]
    pub load_indicator: i32,
    /// Indicates whether form values persist after the action. The default value
    /// is `false`.
    ///
    /// If `true`, form values remain after the action is triggered. To let the
    /// user make changes while the action is being processed, set
    /// [`LoadIndicator`](<https://developers.google.com/workspace/add-ons/reference/rpc/google.apps.card.v1#loadindicator>)
    /// to `NONE`. For [card
    /// messages](<https://developers.google.com/chat/api/guides/v1/messages/create#create>)
    /// in Chat apps, you must also set the action's
    /// [`ResponseType`](<https://developers.google.com/chat/api/reference/rest/v1/spaces.messages#responsetype>)
    /// to `UPDATE_MESSAGE` and use the same
    /// [`card_id`](<https://developers.google.com/chat/api/reference/rest/v1/spaces.messages#CardWithId>)
    /// from the card that contained the action.
    ///
    /// If `false`, the form values are cleared when the action is triggered.
    /// To prevent the user from making changes while the action is being
    /// processed, set
    /// [`LoadIndicator`](<https://developers.google.com/workspace/add-ons/reference/rpc/google.apps.card.v1#loadindicator>)
    /// to `SPINNER`.
    #[prost(bool, tag = "4")]
    pub persist_values: bool,
    /// Optional. Required when opening a
    /// [dialog](<https://developers.google.com/chat/how-tos/dialogs>).
    ///
    /// What to do in response to an interaction with a user, such as a user
    /// clicking a button in a card message.
    ///
    /// If unspecified, the app responds by executing an `action`—like opening a
    /// link or running a function—as normal.
    ///
    /// By specifying an `interaction`, the app can respond in special interactive
    /// ways. For example, by setting `interaction` to `OPEN_DIALOG`, the app can
    /// open a [dialog](<https://developers.google.com/chat/how-tos/dialogs>). When
    /// specified, a loading indicator isn't shown. If specified for
    /// an add-on, the entire card is stripped and nothing is shown in the client.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[prost(enumeration = "action::Interaction", tag = "5")]
    pub interaction: i32,
}
/// Nested message and enum types in `Action`.
pub mod action {
    /// List of string parameters to supply when the action method is invoked.
    /// For example, consider three snooze buttons: snooze now, snooze one day,
    /// or snooze next week. You might use `action method = snooze()`, passing the
    /// snooze type and snooze time in the list of string parameters.
    ///
    /// To learn more, see
    /// [`CommonEventObject`](<https://developers.google.com/chat/api/reference/rest/v1/Event#commoneventobject>).
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionParameter {
        /// The name of the parameter for the action script.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The value of the parameter.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    /// Specifies the loading indicator that the action displays while
    /// making the call to the action.
    ///
    /// [Google Workspace Add-ons and Chat
    /// apps](<https://developers.google.com/workspace/extend>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LoadIndicator {
        /// Displays a spinner to indicate that content is loading.
        Spinner = 0,
        /// Nothing is displayed.
        None = 1,
    }
    impl LoadIndicator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LoadIndicator::Spinner => "SPINNER",
                LoadIndicator::None => "NONE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SPINNER" => Some(Self::Spinner),
                "NONE" => Some(Self::None),
                _ => None,
            }
        }
    }
    /// Optional. Required when opening a
    /// [dialog](<https://developers.google.com/chat/how-tos/dialogs>).
    ///
    /// What to do in response to an interaction with a user, such as a user
    /// clicking a button in a card message.
    ///
    /// If unspecified, the app responds by executing an `action`—like opening a
    /// link or running a function—as normal.
    ///
    /// By specifying an `interaction`, the app can respond in special interactive
    /// ways. For example, by setting `interaction` to `OPEN_DIALOG`, the app can
    /// open a [dialog](<https://developers.google.com/chat/how-tos/dialogs>).
    ///
    /// When specified, a loading indicator isn't shown. If specified for
    /// an add-on, the entire card is stripped and nothing is shown in the client.
    ///
    /// [Google Chat apps](<https://developers.google.com/chat>):
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Interaction {
        /// Default value. The `action` executes as normal.
        Unspecified = 0,
        /// Opens a [dialog](<https://developers.google.com/chat/how-tos/dialogs>), a
        /// windowed, card-based interface that Chat apps use to interact with users.
        ///
        /// Only supported by Chat apps in response to button-clicks on card
        /// messages. If specified for
        /// an add-on, the entire card is stripped and nothing is shown in the
        /// client.
        ///
        /// [Google Chat apps](<https://developers.google.com/chat>):
        OpenDialog = 1,
    }
    impl Interaction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Interaction::Unspecified => "INTERACTION_UNSPECIFIED",
                Interaction::OpenDialog => "OPEN_DIALOG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTERACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "OPEN_DIALOG" => Some(Self::OpenDialog),
                _ => None,
            }
        }
    }
}
