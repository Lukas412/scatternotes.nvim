use nvim_oxi::{
    module,
    Error as OxiError,
    Result as OxiResult,
    api::{
        self,
        types::WindowConfig,
        Window,
        types::WindowRelativeTo
    },
    Function,
    Dictionary
};

#[module]
fn scatternotes() -> OxiResult<Dictionary> {
    // Creates two functions `{open,close}_window` to open and close a
    // floating window.

    let buf = api::create_buf(false, true)?;

    use std::cell::RefCell;
    use std::rc::Rc;

    let win: Rc<RefCell<Option<Window>>> = Rc::default();

    let w = Rc::clone(&win);
    let open_window = Function::from_fn::<_, OxiError>(move |()| {
        if w.borrow().is_some() {
            api::err_writeln("Window is already open");
            return Ok(());
        }

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .height(5)
            .width(10)
            .row(1)
            .col(0)
            .build();

        let mut win = w.borrow_mut();
        *win = Some(api::open_win(&buf, false, &config)?);

        Ok(())
    });

    let close_window = Function::from_fn(move |()| {
        if win.borrow().is_none() {
            api::err_writeln("Window is already closed");
            return Ok(());
        }

        let win = win.borrow_mut().take().unwrap();
        win.close(false)
    });

    Ok(Dictionary::from_iter([
        ("open_window", open_window),
        ("close_window", close_window),
    ]))
}
