use super::{LineType, EPSILON};
use super::geom::{Point};

// Takes unconnected linetypes and attempts to connect them.
pub fn connect_linetypes(mut lines: Vec<LineType>) -> (Vec<LineType>, bool) {
    fn overlap(a: Option<&Point>, b: Option<&Point>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a.close_to(b, EPSILON),
            _ => false
        }
    }

    let mut made_progress = false;
    loop {
        let mut remove_this = None;
        'do_remove: for i in 0 .. lines.len() {
            for k in (i + 1) .. lines.len() {
                let (part_a, part_b) = lines.split_at_mut(i + 1);
                if let (&mut LineType::Unjoined(ref mut a),
                        &mut LineType::Unjoined(ref mut b)) = (&mut part_a[i], &mut part_b[k - i - 1]) {

                    // Aaaaaaaaaaa
                    // Bbbbbb
                    //  ->
                    // bbbbbAaaaaaaaaa
                    if overlap(a.first(), b.first()) {
                        b.reverse();
                        b.pop();
                        b.append(a);
                        remove_this = Some(i);
                        break 'do_remove;
                    }

                    // Aaaaaaaaaaa
                    // bbbbbbbB
                    //  ->
                    // bbbbbbbAaaaaaaaaaaa
                    if overlap(a.first(), b.last()) {
                        b.pop();
                        b.append(a);
                        remove_this = Some(i);
                        break 'do_remove;
                    }

                    // aaaaaaaaaaA
                    // Bbbbbb
                    //  ->
                    //  aaaaaaaaaBbbbbb
                    if overlap(a.last(), b.first()) {
                        a.pop();
                        a.append(b);
                        remove_this = Some(k);
                        break 'do_remove;
                    }
                    // aaaaaaaaA
                    // bbbbbbB
                    //  -> aaaaaaaaAbbbbbbb
                    if overlap(a.last(), b.last()) {
                        b.pop();
                        b.reverse();
                        a.append(b);
                        remove_this = Some(k);
                        break 'do_remove
                    }
                }
            }
        }

        if let Some(p) = remove_this {
            lines.swap_remove(p);
            made_progress = true;
        } else {
            break;
        }
    }

    (lines, made_progress)
}
