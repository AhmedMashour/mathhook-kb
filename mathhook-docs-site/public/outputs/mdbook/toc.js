// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">1.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">2.</strong> Installation</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Calculus</li><li class="chapter-item expanded "><a href="calculus/derivatives.html"><strong aria-hidden="true">3.</strong> Derivatives</a></li><li class="chapter-item expanded "><a href="calculus/integration.html"><strong aria-hidden="true">4.</strong> Integration</a></li><li class="chapter-item expanded "><a href="calculus/limits.html"><strong aria-hidden="true">5.</strong> Limits</a></li><li class="chapter-item expanded "><a href="calculus/series.html"><strong aria-hidden="true">6.</strong> Series</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Algebra</li><li class="chapter-item expanded "><a href="algebra/simplify.html"><strong aria-hidden="true">7.</strong> Simplification</a></li><li class="chapter-item expanded "><a href="algebra/matrix-operations.html"><strong aria-hidden="true">8.</strong> Matrix Operations</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Ordinary Differential Equations</li><li class="chapter-item expanded "><a href="ode/first-order.html"><strong aria-hidden="true">9.</strong> First Order</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="ode/first_order-separable.html"><strong aria-hidden="true">9.1.</strong> Separable</a></li><li class="chapter-item expanded "><a href="ode/first_order-linear.html"><strong aria-hidden="true">9.2.</strong> Linear</a></li><li class="chapter-item expanded "><a href="ode/first_order-bernoulli.html"><strong aria-hidden="true">9.3.</strong> Bernoulli</a></li><li class="chapter-item expanded "><a href="ode/first_order-exact.html"><strong aria-hidden="true">9.4.</strong> Exact</a></li><li class="chapter-item expanded "><a href="ode/first_order-homogeneous.html"><strong aria-hidden="true">9.5.</strong> Homogeneous</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Partial Differential Equations</li><li class="chapter-item expanded "><a href="pde/heat-equation.html"><strong aria-hidden="true">10.</strong> Heat Equation</a></li><li class="chapter-item expanded "><a href="pde/wave-equation.html"><strong aria-hidden="true">11.</strong> Wave Equation</a></li><li class="chapter-item expanded "><a href="pde/laplace-equation.html"><strong aria-hidden="true">12.</strong> Laplace Equation</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Special Functions</li><li class="chapter-item expanded "><a href="special_functions/gamma.html"><strong aria-hidden="true">13.</strong> Gamma Function</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="api-reference.html">API Reference</a></li><li class="chapter-item expanded affix "><a href="performance.html">Performance Guide</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
