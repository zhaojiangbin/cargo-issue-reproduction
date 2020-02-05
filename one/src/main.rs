use pnet::packet::udp::MutableUdpPacket;

fn main() {
    const HDRLEN: usize = MutableUdpPacket::minimum_packet_size();
    println!("{}", HDRLEN);
}
