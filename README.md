# qemu-run
A simple wrapper for QEMU

This tool generates command line options from a YAML file.

Here is a sample YAML file.
```yaml
name: guest
uefi: yes
cpu:
  kvm: yes
  type: host
  cores: 2
memory: 4G
drive:
  - file: /dev/sdb
    format: raw
network:
  - bridge: br0
  - bridge: br1
```

We can convert this YAML to command line option by `qemu-run -d`.
```
$ qemu-run -d sample.yaml
qemu-system-x86_64 -name guest -monitor unix:/tmp/qemu-monitor-guest.sock,server,nowait -bios /usr/share/ovmf/ovmf_code_x64.bin -enable-kvm -cpu host -smp sockets=1,cores=2 -m 4G -drive file=/dev/sdb,format=raw -net nic,vlan=0,macaddr=52:54:ff:be:28:bc -net bridge,vlan=0,br=br0 -net nic,vlan=1,macaddr=52:54:a9:4c:0b:80 -net bridge,vlan=1,br=br1
```
Without `-d (--dry-run)`, it launch a virtual machine.
