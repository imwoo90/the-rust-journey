//! # Client Hooks for Interactive Rendering
//!
//! ## Overview
//! Exposes reactive Dioxus client-side hooks managing external browser libraries.
//! Automates highlight.js syntax highlighting and dynamic Mermaid.js diagram layout.
//!
//! ## Search Tags
//! #hooks, #syntax-highlighting, #mermaid, #client, #dom-observer

use dioxus::prelude::*;

const SYNTAX_HIGHLIGHT_JS: &str = r#"
const highlight = () => {
    if (window.hljs) {
        document.querySelectorAll('pre code:not([data-highlighted="true"])').forEach((el) => {
            window.hljs.highlightElement(el);
            el.setAttribute('data-highlighted', 'true');
        });
    }
};
highlight();
const observer = new MutationObserver(highlight);
observer.observe(document.body, { childList: true, subtree: true });
return () => observer.disconnect();
"#;

const MERMAID_RENDER_JS: &str = r#"
if (window.mermaidObserver) {
    window.mermaidObserver.disconnect();
}
window.mermaidLastIsDark = document.documentElement.classList.contains('dark');

const renderMermaid = () => {
    if (window.mermaid) {
        const isDark = document.documentElement.classList.contains('dark');
        
        if (isDark !== window.mermaidLastIsDark) {
            window.mermaidLastIsDark = isDark;
            document.querySelectorAll('.mermaid').forEach(el => el.remove());
            document.querySelectorAll('pre code.language-mermaid').forEach((el) => {
                el.removeAttribute('data-processed');
                const pre = el.parentElement;
                if (pre) pre.style.display = '';
            });
        }

        const targets = [];
        document.querySelectorAll('pre code.language-mermaid:not([data-processed="true"])').forEach((el) => {
            const pre = el.parentElement;
            if (pre && pre.tagName === 'PRE') {
                const code = el.textContent;
                
                if (pre.nextSibling && pre.nextSibling.className === 'mermaid') {
                    return;
                }

                const container = document.createElement('div');
                container.className = 'mermaid';
                container.style.display = 'flex';
                container.style.justifyContent = 'center';
                container.style.width = '100%';
                container.style.margin = '1.5rem 0';
                container.textContent = code;
                
                el.setAttribute('data-processed', 'true');
                pre.style.display = 'none';
                pre.after(container);
                
                targets.push(container);
            }
        });
        if (targets.length > 0) {
            window.mermaid.initialize({
                startOnLoad: false,
                theme: isDark ? 'base' : 'default',
                securityLevel: 'loose',
                themeVariables: isDark ? {
                    primaryColor: '#1e293b',
                    primaryBorderColor: '#3b82f6',
                    primaryTextColor: '#f8fafc',
                    lineColor: '#94a3b8',
                    arrowheadColor: '#94a3b8',
                    edgeLabelBackground: '#0f172a',
                    textColor: '#e2e8f0',
                    nodeTextColor: '#f8fafc',
                    labelTextColor: '#e2e8f0'
                } : {}
            });
            window.mermaid.run({
                nodes: targets
            });
        }
    }
};
renderMermaid();
window.mermaidObserver = new MutationObserver(renderMermaid);
window.mermaidObserver.observe(document.documentElement, { attributes: true, attributeFilter: ['class'], childList: true, subtree: true });
"#;

/// Hook that observes the DOM and triggers highlight.js on unhighlighted code blocks.
pub fn use_syntax_highlighting() {
    use_effect(move || {
        document::eval(SYNTAX_HIGHLIGHT_JS);
    });
}

/// Hook that dynamically intercepts mermaid code blocks and renders SVG diagrams.
pub fn use_mermaid() {
    use_effect(move || {
        document::eval(MERMAID_RENDER_JS);
    });
}
