use std::path::PathBuf;

use iced::widget::{
    button, column, container, row, scrollable, text, text_input, Space,
};
use iced::{
    Alignment, Color, Element, Length, Task, Theme,
};

use crate::entry::FileItem;
use crate::miller::ColumnState;
use crate::quick_look::QuickLookData;
use crate::sidebar::{get_default_sidebar_items, SidebarItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Icons,
    List,
    Columns,
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(PathBuf),
    NavigateBack,
    NavigateForward,
    SetViewMode(ViewMode),
    SearchChanged(String),
    SelectMillerItem { col_idx: usize, item_idx: usize },
    OpenItem(PathBuf),
    ToggleQuickLook,
    CloseQuickLook,
    ToggleHiddenFiles,
    CreateNewFolder,
    DeleteSelectedItem,
    NoOp,
}

pub struct FileManagerApp {
    current_path: PathBuf,
    history_back: Vec<PathBuf>,
    history_forward: Vec<PathBuf>,
    sidebar_items: Vec<SidebarItem>,
    view_mode: ViewMode,
    columns: Vec<ColumnState>,
    search_query: String,
    show_hidden: bool,
    selected_item: Option<FileItem>,
    quick_look: Option<QuickLookData>,
}

impl Default for FileManagerApp {
    fn default() -> Self {
        Self::new()
    }
}

impl FileManagerApp {
    pub fn new() -> Self {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/"));

        let initial_column = ColumnState::load_directory(&home, false);

        Self {
            current_path: home.clone(),
            history_back: Vec::new(),
            history_forward: Vec::new(),
            sidebar_items: get_default_sidebar_items(),
            view_mode: ViewMode::Columns, // Default to macOS signature Miller Columns
            columns: vec![initial_column],
            search_query: String::new(),
            show_hidden: false,
            selected_item: None,
            quick_look: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(path) => {
                if path.is_dir() {
                    self.history_back.push(self.current_path.clone());
                    self.history_forward.clear();
                    self.current_path = path.clone();
                    self.columns = vec![ColumnState::load_directory(&path, self.show_hidden)];
                    self.selected_item = None;
                    self.quick_look = None;
                }
                Task::none()
            }
            Message::NavigateBack => {
                if let Some(prev) = self.history_back.pop() {
                    self.history_forward.push(self.current_path.clone());
                    self.current_path = prev.clone();
                    self.columns = vec![ColumnState::load_directory(&prev, self.show_hidden)];
                    self.selected_item = None;
                    self.quick_look = None;
                }
                Task::none()
            }
            Message::NavigateForward => {
                if let Some(next) = self.history_forward.pop() {
                    self.history_back.push(self.current_path.clone());
                    self.current_path = next.clone();
                    self.columns = vec![ColumnState::load_directory(&next, self.show_hidden)];
                    self.selected_item = None;
                    self.quick_look = None;
                }
                Task::none()
            }
            Message::SetViewMode(mode) => {
                self.view_mode = mode;
                Task::none()
            }
            Message::SearchChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SelectMillerItem { col_idx, item_idx } => {
                if col_idx < self.columns.len() {
                    // Update selection in current column
                    self.columns[col_idx].selected_index = Some(item_idx);
                    let item = self.columns[col_idx].items[item_idx].clone();
                    self.selected_item = Some(item.clone());

                    // Truncate columns to the right of col_idx
                    self.columns.truncate(col_idx + 1);

                    // If item is a directory, push next Miller column
                    if item.is_dir {
                        self.current_path = item.path.clone();
                        let next_col = ColumnState::load_directory(&item.path, self.show_hidden);
                        self.columns.push(next_col);
                    }
                }
                Task::none()
            }
            Message::OpenItem(path) => {
                if path.is_dir() {
                    return self.update(Message::NavigateTo(path));
                } else {
                    let _ = open::that(&path);
                }
                Task::none()
            }
            Message::ToggleQuickLook => {
                if self.quick_look.is_some() {
                    self.quick_look = None;
                } else if let Some(item) = &self.selected_item {
                    self.quick_look = Some(QuickLookData::load_for(item.clone()));
                }
                Task::none()
            }
            Message::CloseQuickLook => {
                self.quick_look = None;
                Task::none()
            }
            Message::ToggleHiddenFiles => {
                self.show_hidden = !self.show_hidden;
                // Reload active columns
                for col in &mut self.columns {
                    *col = ColumnState::load_directory(&col.dir_path, self.show_hidden);
                }
                Task::none()
            }
            Message::CreateNewFolder => {
                let mut folder_idx = 1;
                let mut new_dir = self.current_path.join("Untitled Folder");
                while new_dir.exists() {
                    new_dir = self.current_path.join(format!("Untitled Folder {}", folder_idx));
                    folder_idx += 1;
                }
                let _ = std::fs::create_dir(&new_dir);
                if let Some(col) = self.columns.last_mut() {
                    *col = ColumnState::load_directory(&col.dir_path, self.show_hidden);
                }
                Task::none()
            }
            Message::DeleteSelectedItem => {
                if let Some(item) = &self.selected_item {
                    let _ = trash::delete(&item.path);
                    self.selected_item = None;
                    if let Some(col) = self.columns.last_mut() {
                        *col = ColumnState::load_directory(&col.dir_path, self.show_hidden);
                    }
                }
                Task::none()
            }
            Message::NoOp => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar = self.render_toolbar();
        let main_area = row![
            self.render_sidebar(),
            container(self.render_content()).width(Length::Fill).height(Length::Fill),
        ]
        .height(Length::Fill);

        let mut content = column![toolbar, main_area].height(Length::Fill);

        if let Some(ql) = &self.quick_look {
            let overlay = self.render_quick_look_modal(ql);
            content = column![content, overlay];
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::from_rgb8(24, 25, 32).into()),
                text_color: Some(Color::from_rgb8(235, 238, 245)),
                ..Default::default()
            })
            .into()
    }

    fn render_toolbar(&self) -> Element<'_, Message> {
        let back_btn = button(text("◀"))
            .on_press_maybe(if !self.history_back.is_empty() {
                Some(Message::NavigateBack)
            } else {
                None
            })
            .padding([4, 8]);

        let fwd_btn = button(text("▶"))
            .on_press_maybe(if !self.history_forward.is_empty() {
                Some(Message::NavigateForward)
            } else {
                None
            })
            .padding([4, 8]);

        let path_text = text(self.current_path.to_string_lossy().to_string())
            .size(13);

        let icon_mode_btn = button(text("⊞ Grid"))
            .style(if self.view_mode == ViewMode::Icons { button::primary } else { button::secondary })
            .on_press(Message::SetViewMode(ViewMode::Icons))
            .padding([4, 8]);

        let list_mode_btn = button(text("☰ List"))
            .style(if self.view_mode == ViewMode::List { button::primary } else { button::secondary })
            .on_press(Message::SetViewMode(ViewMode::List))
            .padding([4, 8]);

        let col_mode_btn = button(text("▥ Columns"))
            .style(if self.view_mode == ViewMode::Columns { button::primary } else { button::secondary })
            .on_press(Message::SetViewMode(ViewMode::Columns))
            .padding([4, 8]);

        let preview_btn = button(text("👁 Preview (Space)"))
            .style(button::secondary)
            .on_press(Message::ToggleQuickLook)
            .padding([4, 8]);

        let new_folder_btn = button(text("➕ Folder"))
            .style(button::secondary)
            .on_press(Message::CreateNewFolder)
            .padding([4, 8]);

        let trash_btn = button(text("🗑 Delete"))
            .style(button::danger)
            .on_press_maybe(self.selected_item.as_ref().map(|_| Message::DeleteSelectedItem))
            .padding([4, 8]);

        let search_box = text_input("Search files...", &self.search_query)
            .on_input(Message::SearchChanged)
            .width(Length::Fixed(180.0))
            .padding(4);

        container(
            row![
                back_btn,
                fwd_btn,
                Space::new().width(10),
                path_text,
                Space::new().width(Length::Fill),
                icon_mode_btn,
                list_mode_btn,
                col_mode_btn,
                Space::new().width(8),
                new_folder_btn,
                trash_btn,
                preview_btn,
                Space::new().width(8),
                search_box,
            ]
            .align_y(Alignment::Center)
            .padding([8, 14])
            .spacing(6),
        )
        .style(|_theme: &Theme| container::Style {
            background: Some(Color::from_rgb8(30, 32, 42).into()),
            border: iced::Border {
                color: Color::from_rgb8(45, 48, 62),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn render_sidebar(&self) -> Element<'_, Message> {
        let mut col = column![
            text("FAVORITES").size(10).style(|_| text::Style { color: Some(Color::from_rgb8(130, 135, 155)) }),
            Space::new().height(4),
        ]
        .spacing(4)
        .padding(12);

        for item in &self.sidebar_items {
            let is_active = self.current_path == item.path;
            let btn = button(
                row![
                    text(item.icon).size(15),
                    text(&item.label).size(13),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([6, 8])
            .style(if is_active { button::primary } else { button::text })
            .on_press(Message::NavigateTo(item.path.clone()));

            col = col.push(btn);
        }

        container(scrollable(col))
            .width(Length::Fixed(190.0))
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::from_rgb8(20, 21, 28).into()),
                border: iced::Border {
                    color: Color::from_rgb8(40, 42, 54),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_content(&self) -> Element<'_, Message> {
        match self.view_mode {
            ViewMode::Columns => self.render_miller_columns(),
            ViewMode::List => self.render_list_view(),
            ViewMode::Icons => self.render_icon_grid(),
        }
    }

    fn render_miller_columns(&self) -> Element<'_, Message> {
        let mut cols_row = row![].spacing(1);

        for (col_idx, col_state) in self.columns.iter().enumerate() {
            let mut items_col = column![].spacing(2).padding(6);

            let filtered_items: Vec<(usize, &FileItem)> = col_state
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    if self.search_query.is_empty() {
                        true
                    } else {
                        item.name.to_lowercase().contains(&self.search_query.to_lowercase())
                    }
                })
                .collect();

            for (orig_idx, item) in filtered_items {
                let is_selected = col_state.selected_index == Some(orig_idx);
                let indicator = if item.is_dir { " ›" } else { "" };

                let item_btn = button(
                    row![
                        text(item.kind.display_icon()).size(14),
                        text(&item.name).size(13).width(Length::Fill),
                        text(indicator).size(14).style(|_| text::Style { color: Some(Color::from_rgb8(150, 155, 175)) }),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(6),
                )
                .width(Length::Fill)
                .padding([5, 8])
                .style(if is_selected { button::primary } else { button::text })
                .on_press(Message::SelectMillerItem {
                    col_idx,
                    item_idx: orig_idx,
                });

                items_col = items_col.push(item_btn);
            }

            let column_container = container(scrollable(items_col))
                .width(Length::Fixed(240.0))
                .height(Length::Fill)
                .style(|_theme: &Theme| container::Style {
                    background: Some(Color::from_rgb8(26, 28, 36).into()),
                    border: iced::Border {
                        color: Color::from_rgb8(40, 43, 56),
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                });

            cols_row = cols_row.push(column_container);
        }

        // Final inspector column if a file is selected
        if let Some(item) = &self.selected_item {
            if !item.is_dir {
                let inspector = container(
                    column![
                        text(item.kind.display_icon()).size(50),
                        Space::new().height(10),
                        text(&item.name).size(15),
                        text(item.format_size()).size(12).style(|_| text::Style { color: Some(Color::from_rgb8(150, 155, 170)) }),
                        Space::new().height(16),
                        button(text("Open in App"))
                            .style(button::primary)
                            .on_press(Message::OpenItem(item.path.clone())),
                        Space::new().height(6),
                        button(text("Quick Look (Space)"))
                            .style(button::secondary)
                            .on_press(Message::ToggleQuickLook),
                    ]
                    .align_x(Alignment::Center)
                    .padding(20),
                )
                .width(Length::Fixed(220.0))
                .height(Length::Fill)
                .style(|_theme: &Theme| container::Style {
                    background: Some(Color::from_rgb8(22, 23, 30).into()),
                    ..Default::default()
                });

                cols_row = cols_row.push(inspector);
            }
        }

        scrollable(cols_row).direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::default(),
            horizontal: scrollable::Scrollbar::default(),
        }).into()
    }

    fn render_list_view(&self) -> Element<'_, Message> {
        let header_row = container(
            row![
                text("Name").size(12).width(Length::FillPortion(4)),
                text("Kind").size(12).width(Length::FillPortion(2)),
                text("Size").size(12).width(Length::FillPortion(2)),
            ]
            .padding([6, 12]),
        )
        .style(|_| container::Style { background: Some(Color::from_rgb8(28, 30, 40).into()), ..Default::default() });

        let mut list_col = column![header_row].spacing(2);

        if let Some(col) = self.columns.first() {
            for item in &col.items {
                let row_btn = button(
                    row![
                        row![text(item.kind.display_icon()).size(14), text(&item.name).size(13)].spacing(8).width(Length::FillPortion(4)),
                        text(format!("{:?}", item.kind)).size(12).width(Length::FillPortion(2)),
                        text(item.format_size()).size(12).width(Length::FillPortion(2)),
                    ]
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([6, 12])
                .style(button::text)
                .on_press(Message::OpenItem(item.path.clone()));

                list_col = list_col.push(row_btn);
            }
        }

        scrollable(list_col).into()
    }

    fn render_icon_grid(&self) -> Element<'_, Message> {
        let mut items_vec: Vec<Element<'_, Message>> = Vec::new();

        if let Some(col) = self.columns.first() {
            for item in &col.items {
                let icon_card = button(
                    column![
                        text(item.kind.display_icon()).size(40),
                        Space::new().height(6),
                        text(&item.name).size(12).align_x(Alignment::Center),
                    ]
                    .align_x(Alignment::Center)
                    .width(Length::Fixed(90.0))
                    .padding(8),
                )
                .style(button::text)
                .on_press(Message::OpenItem(item.path.clone()));

                items_vec.push(icon_card.into());
            }
        }

        let grid = row(items_vec).spacing(16).wrap();
        scrollable(container(grid).padding(20)).into()
    }

    fn render_quick_look_modal<'a>(&self, ql: &'a QuickLookData) -> Element<'a, Message> {
        let preview_body = if let Some(txt) = &ql.text_preview {
            scrollable(text(txt).size(12).style(|_| text::Style { color: Some(Color::from_rgb8(210, 215, 230)) }))
                .height(Length::Fixed(240.0))
        } else {
            scrollable(text(format!("Quick Look preview for: {}", ql.item.name)).size(13))
                .height(Length::Fixed(240.0))
        };

        container(
            column![
                row![
                    text(format!("{} Quick Look Preview", ql.item.kind.display_icon())).size(15),
                    Space::new().width(Length::Fill),
                    button(text("✕ Close")).style(button::secondary).on_press(Message::CloseQuickLook),
                ]
                .align_y(Alignment::Center),
                Space::new().height(10),
                text(&ql.item.name).size(16),
                text(format!("Path: {}", ql.item.path.display())).size(11).style(|_| text::Style { color: Some(Color::from_rgb8(150, 155, 175)) }),
                text(format!("Size: {}", ql.item.format_size())).size(12),
                Space::new().height(10),
                container(preview_body)
                    .padding(10)
                    .style(|_| container::Style {
                        background: Some(Color::from_rgb8(16, 17, 22).into()),
                        border: iced::Border {
                            color: Color::from_rgb8(45, 48, 62),
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                        ..Default::default()
                    }),
                Space::new().height(12),
                row![
                    button(text("Open in Default App"))
                        .style(button::primary)
                        .on_press(Message::OpenItem(ql.item.path.clone())),
                ],
            ]
            .padding(16),
        )
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color::from_rgb8(26, 28, 38).into()),
            border: iced::Border {
                color: Color::from_rgb8(60, 65, 85),
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}
