use toml_edit::Table;

use crate::components::area::InputAreaStyle;
use crate::components::dot::BadgeDotStyle;
use crate::components::item::SelectItemStyle;
use crate::components::options::SelectOptionsStyle;
use crate::components::panel::ColorPanelStyle;
use crate::components::{
    BadgeStyle, BranchStyle, ButtonStyle, CardStyle, CheckboxStyle, CollapseStyle, DividerStyle,
    ImageStyle, InputStyle, LabelStyle, LeafStyle, LinkStyle, LoadingStyle, MenuItemStyle,
    MenuStyle, PaginationStyle, PopupContainerStyle, PopupStyle, ProgressStyle, RadioStyle,
    RateStyle, SelectStyle, SliderStyle, SubMenuStyle, SvgStyle, SwitchStyle, TabbarItemProp,
    TabbarProp, TagStyle, VerificationStyle, ViewStyle,
};
use crate::error::Error;
use crate::prop::manuel::{
    BADGE, BADGE_DOT, BRANCH, BUTTON, CARD, CHECKBOX, COLLAPSE, COLOR_PANEL, DIVIDER, IMAGE, INPUT,
    INPUT_AREA, LABEL, LEAF, LINK, LOADING, MENU, MENU_ITEM, PAGINATION, POPUP, POPUP_CONTAINER,
    PROGRESS, RADIO, RATE, SELECT, SELECT_ITEM, SELECT_OPTIONS, SLIDER, SUB_MENU, SVG, SWITCH,
    TABBAR, TABBAR_ITEM, TAG, VERIFICATION, VIEW,
};
use crate::try_from_toml_item;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelStyle,
    pub view: ViewStyle,
    pub button: ButtonStyle,
    pub card: CardStyle,
    pub radio: RadioStyle,
    pub checkbox: CheckboxStyle,
    pub switch: SwitchStyle,
    pub divider: DividerStyle,
    pub svg: SvgStyle,
    pub image: ImageStyle,
    pub popup: PopupStyle,
    pub popup_container: PopupContainerStyle,
    pub tabbar: TabbarProp,
    pub tabbar_item: TabbarItemProp,
    pub tag: TagStyle,
    pub link: LinkStyle,
    pub menu_item: MenuItemStyle,
    pub sub_menu: SubMenuStyle,
    pub menu: MenuStyle,
    pub collapse: CollapseStyle,
    pub progress: ProgressStyle,
    pub slider: SliderStyle,
    pub loading: LoadingStyle,
    pub color_panel: ColorPanelStyle,
    pub rate: RateStyle,
    pub select_item: SelectItemStyle,
    pub select_options: SelectOptionsStyle,
    pub select: SelectStyle,
    pub badge_dot: BadgeDotStyle,
    pub badge: BadgeStyle,
    pub input_area: InputAreaStyle,
    pub input: InputStyle,
    pub pagination: PaginationStyle,
    pub verification: VerificationStyle,
    pub leaf: LeafStyle,
    pub branch: BranchStyle,
}

try_from_toml_item! {
    ComponentsConf {
        label => LABEL, LabelStyle::default(), |item| item.try_into(),
        view => VIEW, ViewStyle::default(), |item| item.try_into(),
        button => BUTTON, ButtonStyle::default(), |item| item.try_into(),
        card => CARD, CardStyle::default(), |item| item.try_into(),
        radio => RADIO, RadioStyle::default(), |item| item.try_into(),
        checkbox => CHECKBOX, CheckboxStyle::default(), |item| item.try_into(),
        switch => SWITCH, SwitchStyle::default(), |item| item.try_into(),
        divider => DIVIDER, DividerStyle::default(), |item| item.try_into(),
        svg => SVG, SvgStyle::default(), |item| item.try_into(),
        image => IMAGE, ImageStyle::default(), |item| item.try_into(),
        popup => POPUP, PopupStyle::default(), |item| item.try_into(),
        popup_container => POPUP_CONTAINER, PopupContainerStyle::default(), |item| item.try_into(),
        tabbar => TABBAR, TabbarProp::default(), |item| item.try_into(),
        tabbar_item => TABBAR_ITEM, TabbarItemProp::default(), |item| item.try_into(),
        tag => TAG, TagStyle::default(), |item| item.try_into(),
        link => LINK, LinkStyle::default(), |item| item.try_into(),
        menu_item => MENU_ITEM, MenuItemStyle::default(), |item| item.try_into(),
        sub_menu => SUB_MENU, SubMenuStyle::default(), |item| item.try_into(),
        menu => MENU, MenuStyle::default(), |item| item.try_into(),
        collapse => COLLAPSE, CollapseStyle::default(), |item| item.try_into(),
        progress => PROGRESS, ProgressStyle::default(), |item| item.try_into(),
        slider => SLIDER, SliderStyle::default(), |item| item.try_into(),
        loading => LOADING, LoadingStyle::default(), |item| item.try_into(),
        color_panel => COLOR_PANEL, ColorPanelStyle::default(), |item| item.try_into(),
        rate => RATE, RateStyle::default(), |item| item.try_into(),
        select_item => SELECT_ITEM, SelectItemStyle::default(), |item| item.try_into(),
        select_options => SELECT_OPTIONS, SelectOptionsStyle::default(), |item| item.try_into(),
        select => SELECT, SelectStyle::default(), |item| item.try_into(),
        badge_dot => BADGE_DOT, BadgeDotStyle::default(), |item| item.try_into(),
        badge => BADGE, BadgeStyle::default(), |item| item.try_into(),
        input => INPUT, InputStyle::default(), |item| item.try_into(),
        input_area => INPUT_AREA, InputAreaStyle::default(), |item| item.try_into(),
        pagination => PAGINATION, PaginationStyle::default(), |item| item.try_into(),
        verification => VERIFICATION, VerificationStyle::default(), |item| item.try_into(),
        leaf => LEAF, LeafStyle::default(), |item| item.try_into(),
        branch => BRANCH, BranchStyle::default(), |item| item.try_into()
    }, "[components] should be a table"
}

impl From<&ComponentsConf> for Table {
    fn from(value: &ComponentsConf) -> Self {
        let mut table = Table::new();
        table.insert(LABEL, (&value.label).into());
        table.insert(VIEW, (&value.view).into());
        table.insert(BUTTON, (&value.button).into());
        table.insert(CARD, (&value.card).into());
        table.insert(RADIO, (&value.radio).into());
        table.insert(CHECKBOX, (&value.checkbox).into());
        table.insert(SWITCH, (&value.switch).into());
        table.insert(DIVIDER, (&value.divider).into());
        table.insert(SVG, (&value.svg).into());
        table.insert(IMAGE, (&value.image).into());
        table.insert(POPUP, (&value.popup).into());
        table.insert(POPUP_CONTAINER, (&value.popup_container).into());
        table.insert(TABBAR, (&value.tabbar).into());
        table.insert(TABBAR_ITEM, (&value.tabbar_item).into());
        table.insert(TAG, (&value.tag).into());
        table.insert(LINK, (&value.link).into());
        table.insert(MENU_ITEM, (&value.menu_item).into());
        table.insert(SUB_MENU, (&value.sub_menu).into());
        table.insert(MENU, (&value.menu).into());
        table.insert(COLLAPSE, (&value.collapse).into());
        table.insert(SLIDER, (&value.slider).into());
        table.insert(PROGRESS, (&value.progress).into());
        table.insert(LOADING, (&value.loading).into());
        // TODO ... more components
        table
    }
}
