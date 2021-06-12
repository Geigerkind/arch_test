pub use self::wambo::WAMBO;
pub use testo::TESTO;

mod wambo;
mod testo;

fn test() {
    let a = WAMBO;
    let b = wambo::WAMBO;
    let c = TESTO;
    let d = testo::TESTO;
}