# OMG Common Data Representation (CDR) serialization with Serde

Used by [RustDDS](https://github.com/jhelovuo/rustdds/) and [ros2-client](https://github.com/jhelovuo/ros2-client/).

See [Wikipedia](https://en.wikipedia.org/wiki/Common_Data_Representation) or 
[specification in Section "9.3 CDR Transfer Syntax"](https://www.omg.org/spec/CORBA/3.4/Interoperability/PDF).

This is also a part of the full XTYPES [specification](https://www.omg.org/spec/DDS-XTypes/1.2/PDF).
XTYPES specifies several encodings, of which this implemention is only for "plain" CDR.
