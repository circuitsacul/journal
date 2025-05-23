window.updateEditor = (root, text) => {
  root = document.getElementById(root);
  let caret = window.getCaretOffset(root);
  root.innerHTML = text;
  window.restoreCaret(root, caret);
}

window.getCaretOffset = (root) => {  
  // Grab the document-wide Selection object. It represents whatever
  // the user has highlighted (or just the caret if nothing is selected).
  const sel = window.getSelection();

  // If the editor is not focused or there’s no valid range, bail out.
  // Returning 0 is a sane default in that scenario.
  if (!sel || sel.rangeCount === 0) return 0;

  // The first (and usually only) Range in the selection describes
  // the caret: startContainer/Offset == endContainer/Offset.
  const range = sel.getRangeAt(0);

  // Clone the range so we can mutate it without disturbing the real
  // selection that the user sees.
  const pre = range.cloneRange();

  // Make the clone cover the entire *content* of the root element…
  pre.selectNodeContents(root);
  // …then shrink its *end* back to the caret’s actual position.
  pre.setEnd(range.startContainer, range.startOffset);

  // Converting a Range to string concatenates the *visible* text it
  // encloses. The resulting length therefore equals the number of
  // characters from the start of `root` up to the caret.
  return pre.toString().length;
}

window.restoreCaret = (root, index) => {
  let n = index;
  const range = document.createRange();
  range.setStart(root, 0);

  (function walk(node) {
    if (n < 0 || !node) return false;

    if (node.nodeType === Node.TEXT_NODE) {
      if (n <= node.textContent.length) {
        range.setStart(node, n);
        range.collapse(true);
        n = -1;
        return true;
      }
      n -= node.textContent.length;
    }
    for (let i = 0; i < node.childNodes.length; i++) {
      if (walk(node.childNodes[i])) return true;
    }
    return false;
  })(root);

  const sel = window.getSelection();
  sel.removeAllRanges();
  sel.addRange(range);
  root.focus();
}
