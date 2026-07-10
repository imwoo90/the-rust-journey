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

pub fn use_mermaid() {
    use_effect(move || {
        document::eval("
            const renderMermaid = () => {
                if (window.mermaid) {
                    const targets = [];
                    document.querySelectorAll('pre code.language-mermaid:not([data-processed=\"true\"])').forEach((el) => {
                        const pre = el.parentElement;
                        if (pre && pre.tagName === 'PRE') {
                            const code = el.textContent;
                            const container = document.createElement('div');
                            container.className = 'mermaid';
                            container.style.display = 'flex';
                            container.style.justifyContent = 'center';
                            container.style.width = '100%';
                            container.style.margin = '1.5rem 0';
                            container.textContent = code;
                            el.setAttribute('data-processed', 'true');
                            pre.replaceWith(container);
                            targets.push(container);
                        }
                    });
                    if (targets.length > 0) {
                        const isDark = document.documentElement.classList.contains('dark');
                        window.mermaid.initialize({
                            startOnLoad: false,
                            theme: isDark ? 'dark' : 'default',
                            securityLevel: 'loose'
                        });
                        window.mermaid.run({
                            nodes: targets
                        });
                    }
                }
            };
            renderMermaid();
            const observer = new MutationObserver(renderMermaid);
            observer.observe(document.body, { childList: true, subtree: true });
            return () => observer.disconnect();
        ");
    });
}
