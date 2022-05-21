
fn main() {
 let current = get_current_release();

 let bump = get_bump();

 bump_version(current, bump )
}