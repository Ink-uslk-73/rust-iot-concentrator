#[test] fn sections() {
    let s = std::fs::read_to_string("HARDWARE.md").expect("read");
    for sec in ["## 1. Платформа","## 2. CPU","## 3. Память","## 4. Платы расширения","## 5. Хранилище","## 6. Сеть (VLAN)","## 7. Планшет","## 8. GRUB","## 9. Связь железа с софтом"] {
        assert!(s.contains(sec), "missing {}", sec);
    }
}
#[test] fn params() {
    let s = std::fs::read_to_string("HARDWARE.md").expect("read");
    for p in ["Gigabyte WG-C2.R2H.H224-A2","AMD EPYC 7002","24x HDD","isolcpus","mTLS"] { assert!(s.contains(p), "missing {}", p); }
}
#[test] fn vlans() {
    let s = std::fs::read_to_string("HARDWARE.md").expect("read");
    for v in ["VLAN 10","VLAN 20","VLAN 30","VLAN 40","VLAN 50","VLAN 60"] { assert!(s.contains(v), "missing {}", v); }
}
