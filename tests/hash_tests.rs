use delium::{d256, d512, d256c, d512c};

#[test]
fn test_d256() {
    let h = d256("abcdefghijklmnopqrstuvwxyz", 3, 5);
    assert_eq!(h.string, "bdeeb2f01e7e2a0220bd2795f711f5a57fa1e3d103aa38185047b57be6faac93");
}

#[test]
fn test_d512() {
    let h = d512("abcdefghijklmnopqrstuvwxyz", 3, 5);
    assert_eq!(h.string, "e8a8973107efe6870290346983a70ab543c16b6d41c01070d7d913f02276df459b19c3d3721ce85c3af305c50ebf14ca0371bc13f8c5164de338d516a266e1fe");
}

#[test]
fn test_d256c() {
    let h = d256c("abcdefghijklmnopqrstuvwxyz", "2h4usk#5/73uytg#9/#4");
    assert_eq!(h.string, "83174f4e554fc2ff4f6a1456512a41f96d924bb11309567c93f6c743748466ec");
}

#[test]
fn test_d512c() {
    let h = d512c("abcdefghijklmnopqrstuvwxyz", "2h4usk#5/73uytg#9/#4");
    assert_eq!(h.string, "2f0e5b60007e304f072d0735123afc6135ac08be74d7a668458c103ae46166036adf09e9ffee8ae184d6d5a616bcf2e2678d432bf31b99e46ff491c8d6330c08");
}
