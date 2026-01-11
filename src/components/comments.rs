use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Comment {
    pub author: String,
    pub date: String,
    pub avatar_url: String,
    pub text: String,
}

#[component]
pub fn Comments(comments: Vec<Comment>) -> Element {
    rsx! {
        div { class: "mt-12",
            h2 { class: "text-2xl font-bold text-text-dark dark:text-white mb-8", "Comments ({comments.len()})" }
            div { class: "space-y-6",
                for comment in comments {
                    CommentEntry {
                        author: comment.author.clone(),
                        date: comment.date.clone(),
                        avatar_url: comment.avatar_url.clone(),
                        text: comment.text.clone(),
                    }
                }
            }
            div { class: "mt-12",
                h3 { class: "text-lg font-semibold text-text-dark dark:text-white mb-4", "Leave a comment" }
                form { class: "space-y-4",
                    div {
                        textarea {
                            class: "w-full bg-white dark:bg-[#111111] border border-text-dark/10 dark:border-white/10 rounded-xl p-4 text-text-dark dark:text-white placeholder:text-text-dark/40 dark:placeholder:text-gray-600 focus:ring-2 focus:ring-primary focus:border-transparent transition-all outline-none resize-y min-h-[140px]",
                            placeholder: "Write your comment...",
                            rows: 4,
                        }
                    }
                    div { class: "flex justify-end",
                        button {
                            class: "bg-primary-light text-text-dark font-bold py-2.5 px-6 rounded-lg hover:opacity-90 transition-all shadow-lg active:scale-95",
                            "type": "submit",
                            "Post Comment"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CommentEntry(author: String, date: String, avatar_url: String, text: String) -> Element {
    rsx! {
        div { class: "flex gap-4",
            img {
                alt: "{author}",
                class: "size-10 rounded-full object-cover border border-text-dark/10 dark:border-white/10",
                src: "{avatar_url}",
            }
            div { class: "flex-1 bg-text-dark/5 dark:bg-[#252525] rounded-xl p-5 border border-text-dark/5 dark:border-white/5",
                div { class: "flex justify-between items-start mb-2",
                    p { class: "font-semibold text-text-dark dark:text-white text-sm", "{author}" }
                    p { class: "text-xs text-text-dark/60 dark:text-gray-500", "{date}" }
                }
                p { class: "text-text-dark/80 dark:text-gray-300 text-sm leading-relaxed", "{text}" }
            }
        }
    }
}
