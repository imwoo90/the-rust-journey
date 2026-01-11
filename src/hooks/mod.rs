use dioxus::prelude::*;

pub fn use_syntax_highlighting() {
    use_effect(move || {
        document::eval("
            const highlight = () => {
                if (window.hljs) {
                    document.querySelectorAll('pre code:not([data-highlighted=\"true\"])').forEach((el) => {
                        window.hljs.highlightElement(el);
                        el.setAttribute('data-highlighted', 'true');
                    });
                }
            };
            highlight();
            const observer = new MutationObserver(highlight);
            observer.observe(document.body, { childList: true, subtree: true });
            return () => observer.disconnect();
        ");
    });
}
