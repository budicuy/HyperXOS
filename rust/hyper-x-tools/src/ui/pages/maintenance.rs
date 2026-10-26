use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::ui::components::terminal_box;
use crate::ui::theme::*;

pub fn view<'a, Message: Clone + 'a>(
    log_text: &'a str,
    on_clean_cache: Message,
    on_vacuum_journal: Message,
    on_remove_orphans: Message,
    on_rank_mirrors: Message,
    on_fix_keyrings: Message,
    on_clear_log: Message,
) -> Element<'a, Message> {
    let top_bar = column![
        text("Pemeliharaan & Perawatan Sistem")
            .size(16)
            .style(|_| text::Style {
                color: Some(TEXT_MAIN),
            }),
        text("Jaga integritas penyimpanan, bersihkan file sampah dan optimalkan cermin repositori.")
            .size(11)
            .style(|_| text::Style {
                color: Some(TEXT_SUB),
            }),
    ]
    .spacing(2);

    let action_card = |title: &'static str, desc: &'static str, btn_text: &'static str, accent: iced::Color, msg: Message| {
        container(
            row![
                column![
                    text(title).size(13).style(|_| text::Style {
                        color: Some(TEXT_MAIN),
                    }),
                    text(desc).size(11).style(|_| text::Style {
                        color: Some(TEXT_SUB),
                    }),
                ]
                .spacing(2),
                container("").width(Length::Fill),
                button(text(btn_text).size(11))
                    .padding([6, 12])
                    .style(primary_button_style(accent))
                    .on_press(msg),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .padding(14),
        )
        .width(Length::Fill)
        .style(glass_card_style)
    };

    let actions = column![
        action_card(
            "Bersihkan Cache Paket Pacman",
            "Menghapus installer paket lama di /var/cache/pacman/pkg (menyisakan 2 versi terakhir).",
            "Bersihkan Cache",
            ACCENT_CYAN,
            on_clean_cache,
        ),
        action_card(
            "Vakum Log Sistem (Systemd Journal)",
            "Mengompresi dan memangkas ukuran riwayat log sistem hingga di bawah 100 MB.",
            "Vakum Log",
            ACCENT_TEAL,
            on_vacuum_journal,
        ),
        action_card(
            "Hapus Paket Yatim (Orphan Packages)",
            "Mencari dan mencopot dependensi yang sudah tidak lagi digunakan oleh aplikasi manapun.",
            "Copot Orphans",
            ACCENT_AMBER,
            on_remove_orphans,
        ),
        action_card(
            "Urutkan Cermin Repositori Tercepat",
            "Menjalankan Reflector untuk menguji unduhan dan memilih mirror pacman tercepat.",
            "Uji & Urutkan Mirror",
            ACCENT_PURPLE,
            on_rank_mirrors,
        ),
        action_card(
            "Perbaiki & Sinkronisasi Pacman Keyrings",
            "Menginisialisasi ulang kunci GPG archlinux, cachyos, dan hyperxos jika ada error signature.",
            "Perbaiki Kunci GPG",
            ACCENT_GREEN,
            on_fix_keyrings,
        ),
    ]
    .spacing(10);

    let terminal = terminal_box(log_text, on_clear_log);

    let content = column![top_bar, actions, terminal].spacing(14).padding(20);

    scrollable(content).height(Length::Fill).into()
}
