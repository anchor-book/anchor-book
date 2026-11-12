// Expand native details while printing, then restore the reader's choices.
// mdBook's print page joins every chapter, so this runs once per page.
(() => {
    if (window.anchorbookPresentation) {
        return;
    }
    window.anchorbookPresentation = true;
    let closed = [];
    const expand = () => {
        closed = [...document.querySelectorAll('.ab-details:not([open])')];
        closed.forEach(detail => { detail.open = true; });
    };
    if (location.pathname.endsWith('/print.html')) {
        const expandPrintView = () => {
            document.querySelectorAll('.ab-details').forEach(detail => { detail.open = true; });
        };
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', expandPrintView, { once: true });
        } else {
            expandPrintView();
        }
    }
    window.addEventListener('beforeprint', expand);
    window.addEventListener('afterprint', () => {
        closed.forEach(detail => { detail.open = false; });
        closed = [];
    });
})();
