// Presentation only: roadmap content and references remain authored in the Book.
(() => {
    const buttons = Array.from(document.querySelectorAll('.roadmap-toggle'));
    function setExpanded(button, expanded) {
        const details = document.getElementById(button.getAttribute('aria-controls'));
        if (!details) return;
        button.setAttribute('aria-expanded', String(expanded));
        details.hidden = !expanded;
        button.querySelector('.roadmap-toggle-icon').textContent = expanded ? '▾' : '▸';
    }
    for (const button of buttons) {
        setExpanded(button, false);
        button.addEventListener('click', () => {
            setExpanded(button, button.getAttribute('aria-expanded') !== 'true');
        });
    }
    function revealTarget() {
        let id;
        try { id = decodeURIComponent(location.hash.slice(1)); } catch { return; }
        const target = document.getElementById(id);
        if (!target) return;
        const button = target.querySelector('.roadmap-toggle') ||
            buttons.find(item => item.getAttribute('aria-controls') === id);
        if (button) setExpanded(button, true);
    }
    revealTarget();
    window.addEventListener('hashchange', revealTarget);
})();
