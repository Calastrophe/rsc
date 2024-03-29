type FileData = Vec<u8>;

// Taken from this work, but modified with cfg-if and converting to String not Vec<u8>
// https://github.com/kirjavascript/trueLMAO/blob/3bab516e577359cb8374a381dd803a651632fcad/frontend/src/widgets/file.rs

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use js_sys::{ArrayBuffer, Uint8Array};
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::{window, FileReader, HtmlInputElement};

        pub struct FileDialog {
            tx: std::sync::mpsc::Sender<FileData>,
            rx: std::sync::mpsc::Receiver<FileData>,
            input: HtmlInputElement,
            closure: Option<Closure<dyn FnMut()>>,
        }

        impl Default for FileDialog {
            fn default() -> Self {
                let (tx, rx) = std::sync::mpsc::channel();

                let document = window().unwrap().document().unwrap();
                let body = document.body().unwrap();
                let input = document
                    .create_element("input")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();
                input.set_attribute("type", "file").unwrap();
                input.style().set_property("display", "none").unwrap();
                body.append_child(&input).unwrap();

                Self {
                    rx,
                    tx,
                    input,
                    closure: None,
                }
            }
        }

        impl Drop for FileDialog {
            fn drop(&mut self) {
                self.input.remove();
                if self.closure.is_some() {
                    std::mem::replace(&mut self.closure, None).unwrap().forget();
                }
            }
        }

        impl FileDialog {
            pub fn open(&mut self) {
                if let Some(closure) = &self.closure {
                    self.input
                        .remove_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                        .unwrap();
                    std::mem::replace(&mut self.closure, None).unwrap().forget();
                }

                let tx = self.tx.clone();
                let input_clone = self.input.clone();

                let closure = Closure::once(move || {
                    if let Some(file) = input_clone.files().and_then(|files| files.get(0)) {
                        let reader = FileReader::new().unwrap();
                        let reader_clone = reader.clone();
                        let onload_closure = Closure::once(Box::new(move || {
                            let array_buffer = reader_clone
                                .result()
                                .unwrap()
                                .dyn_into::<ArrayBuffer>()
                                .unwrap();
                            let buffer = Uint8Array::new(&array_buffer).to_vec();
                            tx.send(buffer).ok();
                        }));

                        reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                        reader.read_as_array_buffer(&file).unwrap();
                        onload_closure.forget();
                    }
                });

                self.input
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                    .unwrap();
                self.closure = Some(closure);
                self.input.click();
            }

            pub fn get(&self) -> Option<String> {
                if let Ok(file) = self.rx.try_recv() {
                    Some(String::from_utf8_lossy(&file).to_string())
                } else {
                    None
                }
            }
        }
} else {
        use rfd;

        pub struct FileDialog {
            file: Option<String>,
        }

        impl Default for FileDialog {
            fn default() -> Self {
                Self { file: None }
            }
        }

        impl FileDialog {
            pub fn open(&mut self) {
                let path = rfd::FileDialog::new().pick_file();
                if let Some(path) = path {
                    self.file = std::fs::read_to_string(path).ok()
                }
            }

            pub fn get(&mut self) -> Option<String> {
                std::mem::replace(&mut self.file, None)
            }
        }
    }
}
