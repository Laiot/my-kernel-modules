use kernel::prelude::*;

module! {
    type: HelloKernel,
    name: b"hello-kernel",
    author: b"Carmelo Sarta",
    description: b"Rust minimal hello world",
    license: b"GPL",
}

struct HelloKernel {
    words: Vec<String>,
}

impl kernel::Module for HelloKernel {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust minimal sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let mut words = Vec::new();
        words.try_push("Hello")?;
        words.try_push("there,")?;
        words.try_push("Kernel!")?;

        Ok(HelloKernel { words.try_to_owned()? })
    }
}

impl Drop for HelloKernel {
    fn drop(&mut self) {
        pr_info!("My phrase is {:?}\n", self.words.join("-"));
        pr_info!("hello-kernel (exit)\n");
    }
}