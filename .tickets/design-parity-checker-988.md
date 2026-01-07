---
id: design-parity-checker-988
status: closed
deps: [design-parity-checker-4v9]
links: []
created: 2025-12-13T21:40:02.752778+01:00
type: task
priority: 2
---
# Implement OCR text extraction via Tesseract

Use Tesseract (via Rust bindings or subprocess) to extract text blocks from screenshots when no DOM/Figma tree exists. Return OcrBlock[] with text content and bounding boxes. Only invoke when content metrics requested.


