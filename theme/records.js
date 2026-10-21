// Expand native details while printing, then restore the reader's choices.
(() => {
    let closed = [];
    const expand = () => {
        closed = [...document.querySelectorAll('.ab-details:not([open])')];
        closed.forEach(detail => { detail.open = true; });
    };
    if (location.pathname.endsWith('/print.html')) {
        document.querySelectorAll('.ab-details').forEach(detail => { detail.open = true; });
    }
    window.addEventListener('beforeprint', expand);
    window.addEventListener('afterprint', () => {
        closed.forEach(detail => { detail.open = false; });
        closed = [];
    });
})();
