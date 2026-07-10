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
            if (window.mermaidObserver) {
                window.mermaidObserver.disconnect();
            }
            window.mermaidLastIsDark = document.documentElement.classList.contains('dark');
            
            const renderMermaid = () => {
                if (window.mermaid) {
                    const isDark = document.documentElement.classList.contains('dark');
                    
                    // If dark/light mode changed since last render, clear old SVGs to re-render
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
                    document.querySelectorAll('pre code.language-mermaid:not([data-processed=\"true\"])').forEach((el) => {
                        const pre = el.parentElement;
                        if (pre && pre.tagName === 'PRE') {
                            const code = el.textContent;
                            
                            // Check if next sibling is already a mermaid div to avoid duplicate creation
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
                            
                            // Hide original pre instead of replacing it, keeping VDOM node tree intact
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
                                primaryColor: '#1e293b',          // slate-800
                                primaryBorderColor: '#3b82f6',    // blue-500
                                primaryTextColor: '#f8fafc',      // slate-50
                                lineColor: '#94a3b8',             // slate-400
                                arrowheadColor: '#94a3b8',        // slate-400
                                edgeLabelBackground: '#0f172a',   // slate-950 (dark background match)
                                textColor: '#e2e8f0',             // slate-200
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
        ");
    });
}
