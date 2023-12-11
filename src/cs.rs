use critical_section::{set_impl, Impl, RawRestoreState};

struct SingleCoreCriticalSection;
set_impl!(SingleCoreCriticalSection);

unsafe impl Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        // TODO:
        true
    }

    unsafe fn release(_was_active: RawRestoreState) {
        // TODO:
    }
}
