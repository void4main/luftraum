# Luftraum
(Project for me to learn Rust and Bevy.)

Application to show live air traffic in 3D.
* Uses SRTM terrain data to show environment
* and dump1090 to get local ADS-B plane data

Start `./dump1090 --device-type hackrf --net-sbs-port 30003` on localhost.
Luftraum connects to 127.0.0.1:30003 statically by now.

![Luftraum](https://github.com/void4main/luftraum/blob/master/luftraum-screenshot-0.1.png)
![Luftraum](https://github.com/void4main/luftraum/blob/master/luftraum-screenshot-0.1b.png)

I got my SRTM ASCII data here:
### SRTM data source
Jarvis A., H.I. Reuter, A.  Nelson, E. Guevara, 2008, Hole-filled  seamless SRTM
data V4, International  Centre for Tropical  Agriculture (CIAT), available  from
http://srtm.csi.cgiar.org

REFERENCES

Reuter  H.I,  A.  Nelson,  A.  Jarvis,  2007,  An  evaluation  of  void  filling
interpolation  methods  for  SRTM  data,  International  Journal  of  Geographic
Information Science, 21:9, 983-1008.