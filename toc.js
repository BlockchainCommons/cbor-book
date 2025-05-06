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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">Introduction</li><li class="chapter-item expanded "><a href="introduction/cover.html"><strong aria-hidden="true">1.</strong> Cover</a></li><li class="chapter-item expanded "><a href="introduction/colophon.html"><strong aria-hidden="true">2.</strong> Colophon</a></li><li class="chapter-item expanded "><a href="introduction/introduction.html"><strong aria-hidden="true">3.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="introduction/from_xml_to_json_to_cbor.html"><strong aria-hidden="true">4.</strong> From XML to JSON to CBOR</a></li><li class="chapter-item expanded "><a href="introduction/cbor_vs_the_other_guys.html"><strong aria-hidden="true">5.</strong> CBOR vs. the Other Guys</a></li><li class="chapter-item expanded affix "><li class="part-title">Part I: CBOR</li><li class="chapter-item expanded "><a href="part_1/practical_introduction_to_cbor.html"><strong aria-hidden="true">6.</strong> A Practical Introduction to CBOR</a></li><li class="chapter-item expanded "><a href="part_1/cbor_tags.html"><strong aria-hidden="true">7.</strong> Extending Semantics with CBOR Tags</a></li><li class="chapter-item expanded "><a href="part_1/indefinite_length_items.html"><strong aria-hidden="true">8.</strong> Indefinite-Length Items</a></li><li class="chapter-item expanded "><a href="part_1/cbor_sequences.html"><strong aria-hidden="true">9.</strong> Sequences: Streaming Independent Data Items</a></li><li class="chapter-item expanded "><a href="part_1/cbor_schemas_with_cddl.html"><strong aria-hidden="true">10.</strong> CBOR Schemas with CDDL</a></li><li class="chapter-item expanded affix "><li class="part-title">Part II: dCBOR</li><li class="chapter-item expanded "><a href="part_2/determinism.html"><strong aria-hidden="true">11.</strong> Determinism: Why Consistent Encodings Matter</a></li><li class="chapter-item expanded "><a href="part_2/cbor_cde_dcbor.html"><strong aria-hidden="true">12.</strong> From CBOR, to CDE, to dCBOR</a></li><li class="chapter-item expanded "><a href="part_2/using_dcbor.html"><strong aria-hidden="true">13.</strong> Using dCBOR</a></li><li class="chapter-item expanded "><a href="part_2/dcbor_tags.html"><strong aria-hidden="true">14.</strong> dCBOR Tags</a></li><li class="chapter-item expanded affix "><li class="part-title">Part III: Gordian Envelope</li><li class="chapter-item expanded "><a href="part_3/introducing_gordian_envelope.html"><strong aria-hidden="true">15.</strong> Introducing Gordian Envelope</a></li><li class="chapter-item expanded "><a href="part_3/envelope_semantics_and_structure.html"><strong aria-hidden="true">16.</strong> Envelope Semantics and Structure</a></li><li class="chapter-item expanded "><a href="part_3/envelope_encoding_and_processing.html"><strong aria-hidden="true">17.</strong> Envelope Encoding and Processing</a></li><li class="chapter-item expanded "><a href="part_3/practical_applications_and_patterns.html"><strong aria-hidden="true">18.</strong> Practical Applications and Patterns</a></li><li class="chapter-item expanded "><a href="part_3/tooling_and_libraries.html"><strong aria-hidden="true">19.</strong> Tooling and Libraries</a></li><li class="chapter-item expanded "><a href="part_3/advanced_topics.html"><strong aria-hidden="true">20.</strong> Advanced Topics</a></li><li class="chapter-item expanded affix "><li class="part-title">Appendices</li><li class="chapter-item expanded "><a href="appendices/cbor_header_bytes.html"><strong aria-hidden="true">21.</strong> Appendix A: CBOR Header Bytes</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
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
