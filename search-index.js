var searchIndex={};
searchIndex["tiberius"] = {"doc":"A pure-rust TDS implementation for Microsoft SQL Server…","i":[[3,"Uuid","tiberius","A Universally Unique Identifier (UUID).",null,null],[3,"Client","","`Client` is the main entry point to the SQL Server,…",null,null],[3,"ClientBuilder","","A builder for creating a new [`Client`].",null,null],[3,"QueryResult","","A set of `Streams` of [`Rows`] resulting from a `SELECT`…",null,null],[3,"ExecuteResult","","A `Stream` of counts of affected rows resulting from an…",null,null],[3,"Column","","",null,null],[3,"Row","","A row of data from a query.",null,null],[4,"AuthMethod","","",null,null],[13,"SqlServer","","Authenticate directly with SQL Server. The only…",0,null],[13,"Windows","","Authenticate with Windows credentials. Only available on…",0,null],[13,"WindowsIntegrated","","Authenticate as the currently logged in user. Only…",0,null],[4,"Error","","A unified error enum that contains several errors that…",null,null],[13,"Io","","",1,null],[13,"Protocol","","",1,null],[13,"Encoding","","",1,null],[13,"Conversion","","",1,null],[13,"Utf8","","",1,null],[13,"Utf16","","",1,null],[13,"ParseInt","","",1,null],[13,"Server","","",1,null],[13,"Canceled","","",1,null],[13,"Tls","","",1,null],[4,"EncryptionLevel","","The configured encryption level specifying if encryption…",null,null],[13,"Off","","Only use encryption for the login procedure",2,null],[13,"On","","Encrypt everything if possible",2,null],[13,"NotSupported","","Do not encrypt anything",2,null],[13,"Required","","Encrypt everything and fail if not possible",2,null],[11,"host","","A host or ip address to connect to.",3,[[["self"]]]],[11,"port","","The server port.",3,[[["u16"],["self"]]]],[11,"database","","The database to connect to.",3,[[["self"]]]],[11,"instance_name","","The instance name as defined in the SQL Browser. Only…",3,[[["self"]]]],[11,"encryption","","Set the preferred encryption level.",3,[[["self"],["encryptionlevel"]]]],[11,"trust_cert","","If set, the server certificate will not be validated and…",3,[[["self"]]]],[11,"authentication","","Sets the authentication method.",3,[[["self"],["authmethod"]]]],[11,"build","","Creates a new client and connects to the server.",3,[[]]],[11,"from_ado_string","","Creates a new `ClientBuilder` from an ADO.NET connection…",3,[[["str"]],["result"]]],[11,"sql_server","","Construct a new SQL Server authentication configuration.",0,[[],["self"]]],[11,"windows","","Construct a new Windows authentication configuration. Only…",0,[[],["self"]]],[11,"builder","","Starts an instance of [`ClientBuilder`] for specifying the…",4,[[],["clientbuilder"]]],[11,"execute","","Executes SQL statements in the SQL Server, returning the…",4,[[["self"]]]],[11,"query","","Executes SQL statements in the SQL Server, returning…",4,[[["self"]]]],[11,"columns","","Names of the columns of the current resultset. Order is…",5,[[["self"]],[["str"],["vec",["str"]]]]],[11,"next_resultset","","Returns `true` if stream has more result sets available.…",5,[[["self"]],["bool"]]],[11,"into_vec","","Collects results from all queries in the stream into…",5,[[]]],[11,"into_first","","A convenience method on collecting the results of the…",5,[[]]],[11,"total","","Aggregates all resulting row counts into a sum.",6,[[],["u64"]]],[11,"name","","",7,[[["self"]],["str"]]],[11,"columns","","Columns defining the row data. Columns listed here are in…",8,[[["self"]]]],[11,"len","","Returns the amount of columns in the row",8,[[["self"]],["usize"]]],[11,"get","","Retrieve a column's value for a given column index",8,[[["i"],["self"]],["r"]]],[11,"try_get","","Retrieve a column's value for a given column index.",8,[[["i"],["self"]],[["result",["option"]],["option"]]]],[11,"into_first","","Takes the first column data out from the row, consuming…",8,[[],[["result",["option"]],["option"]]]],[0,"numeric","","Representations of numeric types.",null,null],[3,"Numeric","tiberius::numeric","Represent a sql Decimal / Numeric type. It is stored in a…",null,null],[0,"time","tiberius","Date and time handling.",null,null],[3,"DateTime","tiberius::time","A presentation of `datetime` type in the server.",null,null],[3,"SmallDateTime","","A presentation of `smalldatetime` type in the server.",null,null],[3,"Date","","A presentation of `date` type in the server.",null,null],[3,"Time","","A presentation of `time` type in the server.",null,null],[3,"DateTime2","","A presentation of `datetime2` type in the server.",null,null],[3,"DateTimeOffset","","A presentation of `datetimeoffset` type in the server.",null,null],[0,"xml","tiberius","",null,null],[3,"XmlSchema","tiberius::xml","Provides information of the location for the schema.",null,null],[3,"XmlData","","A representation of XML data in TDS. Holds the data as a…",null,null],[6,"Result","tiberius","",null,null],[8,"ToSql","","",null,null],[10,"to_sql","","",9,[[["self"]]]],[11,"from","","",10,[[["t"]],["t"]]],[11,"into","","",10,[[],["u"]]],[11,"to_owned","","",10,[[["self"]],["t"]]],[11,"clone_into","","",10,[[["self"],["t"]]]],[11,"to_string","","",10,[[["self"]],["string"]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"try_into","","",10,[[],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"type_id","","",10,[[["self"]],["typeid"]]],[11,"vzip","","",10,[[],["v"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[],["u"]]],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"try_into","","",4,[[],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"type_id","","",4,[[["self"]],["typeid"]]],[11,"vzip","","",4,[[],["v"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[],["u"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,[[["self"],["t"]]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"try_into","","",3,[[],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"type_id","","",3,[[["self"]],["typeid"]]],[11,"vzip","","",3,[[],["v"]]],[11,"from","","",5,[[["t"]],["t"]]],[11,"into","","",5,[[],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"try_into","","",5,[[],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"type_id","","",5,[[["self"]],["typeid"]]],[11,"try_poll_next","","",5,[[["context"],["pin"],["s"]],[["poll",["option"]],["option",["result"]]]]],[11,"vzip","","",5,[[],["v"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"into_iter","","",6,[[],["i"]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"vzip","","",6,[[],["v"]]],[11,"from","","",7,[[["t"]],["t"]]],[11,"into","","",7,[[],["u"]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"try_into","","",7,[[],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"type_id","","",7,[[["self"]],["typeid"]]],[11,"vzip","","",7,[[],["v"]]],[11,"from","","",8,[[["t"]],["t"]]],[11,"into","","",8,[[],["u"]]],[11,"into_iter","","",8,[[],["i"]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"try_into","","",8,[[],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"type_id","","",8,[[["self"]],["typeid"]]],[11,"vzip","","",8,[[],["v"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,[[["self"],["t"]]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,[[["self"],["t"]]]],[11,"to_string","","",1,[[["self"]],["string"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"vzip","","",1,[[],["v"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,[[["self"],["t"]]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"vzip","","",2,[[],["v"]]],[11,"from","tiberius::numeric","",11,[[["t"]],["t"]]],[11,"into","","",11,[[],["u"]]],[11,"to_owned","","",11,[[["self"]],["t"]]],[11,"clone_into","","",11,[[["self"],["t"]]]],[11,"to_string","","",11,[[["self"]],["string"]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"try_into","","",11,[[],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"type_id","","",11,[[["self"]],["typeid"]]],[11,"vzip","","",11,[[],["v"]]],[11,"from","tiberius::time","",12,[[["t"]],["t"]]],[11,"into","","",12,[[],["u"]]],[11,"to_owned","","",12,[[["self"]],["t"]]],[11,"clone_into","","",12,[[["self"],["t"]]]],[11,"try_from","","",12,[[["u"]],["result"]]],[11,"try_into","","",12,[[],["result"]]],[11,"borrow","","",12,[[["self"]],["t"]]],[11,"borrow_mut","","",12,[[["self"]],["t"]]],[11,"type_id","","",12,[[["self"]],["typeid"]]],[11,"vzip","","",12,[[],["v"]]],[11,"from","","",13,[[["t"]],["t"]]],[11,"into","","",13,[[],["u"]]],[11,"to_owned","","",13,[[["self"]],["t"]]],[11,"clone_into","","",13,[[["self"],["t"]]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"try_into","","",13,[[],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"type_id","","",13,[[["self"]],["typeid"]]],[11,"vzip","","",13,[[],["v"]]],[11,"from","","",14,[[["t"]],["t"]]],[11,"into","","",14,[[],["u"]]],[11,"to_owned","","",14,[[["self"]],["t"]]],[11,"clone_into","","",14,[[["self"],["t"]]]],[11,"try_from","","",14,[[["u"]],["result"]]],[11,"try_into","","",14,[[],["result"]]],[11,"borrow","","",14,[[["self"]],["t"]]],[11,"borrow_mut","","",14,[[["self"]],["t"]]],[11,"type_id","","",14,[[["self"]],["typeid"]]],[11,"vzip","","",14,[[],["v"]]],[11,"from","","",15,[[["t"]],["t"]]],[11,"into","","",15,[[],["u"]]],[11,"to_owned","","",15,[[["self"]],["t"]]],[11,"clone_into","","",15,[[["self"],["t"]]]],[11,"try_from","","",15,[[["u"]],["result"]]],[11,"try_into","","",15,[[],["result"]]],[11,"borrow","","",15,[[["self"]],["t"]]],[11,"borrow_mut","","",15,[[["self"]],["t"]]],[11,"type_id","","",15,[[["self"]],["typeid"]]],[11,"vzip","","",15,[[],["v"]]],[11,"from","","",16,[[["t"]],["t"]]],[11,"into","","",16,[[],["u"]]],[11,"to_owned","","",16,[[["self"]],["t"]]],[11,"clone_into","","",16,[[["self"],["t"]]]],[11,"try_from","","",16,[[["u"]],["result"]]],[11,"try_into","","",16,[[],["result"]]],[11,"borrow","","",16,[[["self"]],["t"]]],[11,"borrow_mut","","",16,[[["self"]],["t"]]],[11,"type_id","","",16,[[["self"]],["typeid"]]],[11,"vzip","","",16,[[],["v"]]],[11,"from","","",17,[[["t"]],["t"]]],[11,"into","","",17,[[],["u"]]],[11,"to_owned","","",17,[[["self"]],["t"]]],[11,"clone_into","","",17,[[["self"],["t"]]]],[11,"try_from","","",17,[[["u"]],["result"]]],[11,"try_into","","",17,[[],["result"]]],[11,"borrow","","",17,[[["self"]],["t"]]],[11,"borrow_mut","","",17,[[["self"]],["t"]]],[11,"type_id","","",17,[[["self"]],["typeid"]]],[11,"vzip","","",17,[[],["v"]]],[11,"from","tiberius::xml","",18,[[["t"]],["t"]]],[11,"into","","",18,[[],["u"]]],[11,"to_owned","","",18,[[["self"]],["t"]]],[11,"clone_into","","",18,[[["self"],["t"]]]],[11,"try_from","","",18,[[["u"]],["result"]]],[11,"try_into","","",18,[[],["result"]]],[11,"borrow","","",18,[[["self"]],["t"]]],[11,"borrow_mut","","",18,[[["self"]],["t"]]],[11,"type_id","","",18,[[["self"]],["typeid"]]],[11,"vzip","","",18,[[],["v"]]],[11,"from","","",19,[[["t"]],["t"]]],[11,"into","","",19,[[],["u"]]],[11,"to_owned","","",19,[[["self"]],["t"]]],[11,"clone_into","","",19,[[["self"],["t"]]]],[11,"to_string","","",19,[[["self"]],["string"]]],[11,"try_from","","",19,[[["u"]],["result"]]],[11,"try_into","","",19,[[],["result"]]],[11,"borrow","","",19,[[["self"]],["t"]]],[11,"borrow_mut","","",19,[[["self"]],["t"]]],[11,"type_id","","",19,[[["self"]],["typeid"]]],[11,"vzip","","",19,[[],["v"]]],[11,"partial_cmp","tiberius","",10,[[["self"],["uuid"]],[["ordering"],["option",["ordering"]]]]],[11,"lt","","",10,[[["self"],["uuid"]],["bool"]]],[11,"le","","",10,[[["self"],["uuid"]],["bool"]]],[11,"gt","","",10,[[["self"],["uuid"]],["bool"]]],[11,"ge","","",10,[[["self"],["uuid"]],["bool"]]],[11,"fmt","","",10,[[["formatter"],["self"]],[["error"],["result",["error"]]]]],[11,"fmt","","",10,[[["formatter"],["self"]],[["error"],["result",["error"]]]]],[11,"fmt","","",10,[[["formatter"],["self"]],[["error"],["result",["error"]]]]],[11,"default","","",10,[[],["uuid"]]],[11,"eq","","",10,[[["self"],["uuid"]],["bool"]]],[11,"ne","","",10,[[["self"],["uuid"]],["bool"]]],[11,"clone","","",10,[[["self"]],["uuid"]]],[11,"hash","","",10,[[["self"],["__h"]]]],[11,"fmt","","",10,[[["formatter"],["self"]],[["error"],["result",["error"]]]]],[11,"cmp","","",10,[[["self"],["uuid"]],["ordering"]]],[11,"from_str","","",10,[[["str"]],[["result",["uuid"]],["uuid"]]]],[11,"to_sql","","",10,[[["self"]]]],[11,"to_sql","tiberius::xml","",19,[[["self"]]]],[11,"as_ref","","",19,[[["self"]],["str"]]],[11,"from","tiberius","",1,[[["error"]],["self"]]],[11,"from","","",1,[[["error"]],["self"]]],[11,"from","","",1,[[["infallible"]],["self"]]],[11,"from","","",1,[[["error"]],["error"]]],[11,"from","","",1,[[["parseinterror"]],["error"]]],[11,"from","","",1,[[["utf8error"]],["error"]]],[11,"from","","",1,[[["fromutf8error"]],["error"]]],[11,"from","","",1,[[["fromutf16error"]],["error"]]],[11,"into_iter","","",6,[[]]],[11,"into_iter","","",8,[[]]],[11,"clone","","",3,[[["self"]],["clientbuilder"]]],[11,"clone","","",0,[[["self"]],["authmethod"]]],[11,"clone","","",1,[[["self"]],["error"]]],[11,"clone","tiberius::numeric","",11,[[["self"]],["numeric"]]],[11,"clone","tiberius::time","",12,[[["self"]],["datetime"]]],[11,"clone","","",13,[[["self"]],["smalldatetime"]]],[11,"clone","","",14,[[["self"]],["date"]]],[11,"clone","","",15,[[["self"]],["time"]]],[11,"clone","","",16,[[["self"]],["datetime2"]]],[11,"clone","","",17,[[["self"]],["datetimeoffset"]]],[11,"clone","tiberius::xml","",18,[[["self"]],["xmlschema"]]],[11,"clone","","",19,[[["self"]],["xmldata"]]],[11,"clone","tiberius","",2,[[["self"]],["encryptionlevel"]]],[11,"default","","",3,[[],["self"]]],[11,"eq","","",0,[[["authmethod"],["self"]],["bool"]]],[11,"ne","","",0,[[["authmethod"],["self"]],["bool"]]],[11,"eq","tiberius::numeric","",11,[[["self"]],["bool"]]],[11,"eq","tiberius::time","",12,[[["self"],["datetime"]],["bool"]]],[11,"ne","","",12,[[["self"],["datetime"]],["bool"]]],[11,"eq","","",13,[[["smalldatetime"],["self"]],["bool"]]],[11,"ne","","",13,[[["smalldatetime"],["self"]],["bool"]]],[11,"eq","","",14,[[["date"],["self"]],["bool"]]],[11,"ne","","",14,[[["date"],["self"]],["bool"]]],[11,"eq","","",15,[[["self"],["time"]],["bool"]]],[11,"eq","","",16,[[["datetime2"],["self"]],["bool"]]],[11,"ne","","",16,[[["datetime2"],["self"]],["bool"]]],[11,"eq","","",17,[[["datetimeoffset"],["self"]],["bool"]]],[11,"ne","","",17,[[["datetimeoffset"],["self"]],["bool"]]],[11,"eq","tiberius::xml","",18,[[["self"],["xmlschema"]],["bool"]]],[11,"ne","","",18,[[["self"],["xmlschema"]],["bool"]]],[11,"eq","","",19,[[["xmldata"],["self"]],["bool"]]],[11,"ne","","",19,[[["xmldata"],["self"]],["bool"]]],[11,"eq","tiberius","",2,[[["encryptionlevel"],["self"]],["bool"]]],[11,"to_string","tiberius::xml","",19,[[["self"]],["string"]]],[11,"fmt","tiberius","",0,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",1,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",7,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",8,[[["formatter"],["self"]],["result"]]],[11,"fmt","tiberius::numeric","",11,[[["self"],["formatter"]],[["error"],["result",["error"]]]]],[11,"fmt","tiberius::time","",12,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",13,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",14,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",15,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",16,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",17,[[["formatter"],["self"]],["result"]]],[11,"fmt","tiberius::xml","",18,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",19,[[["formatter"],["self"]],["result"]]],[11,"fmt","tiberius","",2,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",1,[[["formatter"],["self"]],["result"]]],[11,"fmt","tiberius::numeric","",11,[[["self"],["formatter"]],[["error"],["result",["error"]]]]],[11,"try_from","tiberius","",2,[[["u8"]],[["result",["encryptionlevel"]],["encryptionlevel"]]]],[11,"try_from","","",2,[[["u32"]],[["result",["encryptionlevel"]],["encryptionlevel"]]]],[11,"poll_next","","",5,[[["self"],["context"],["pin"]],[["poll",["option"]],["option"]]]],[11,"nil","","The 'nil UUID'.",10,[[],["uuid"]]],[11,"from_fields","","Creates a UUID from four field values in big-endian order.",10,[[["u16"],["u32"]],[["result",["uuid","error"]],["uuid"],["error"]]]],[11,"from_fields_le","","Creates a UUID from four field values in little-endian…",10,[[["u16"],["u32"]],[["result",["uuid","error"]],["uuid"],["error"]]]],[11,"from_u128","","Creates a UUID from a 128bit value in big-endian order.",10,[[["u128"]],["uuid"]]],[11,"from_u128_le","","Creates a UUID from a 128bit value in little-endian order.",10,[[["u128"]],["uuid"]]],[11,"from_slice","","Creates a UUID using the supplied big-endian bytes.",10,[[],[["result",["uuid","error"]],["uuid"],["error"]]]],[11,"from_bytes","","Creates a UUID using the supplied big-endian bytes.",10,[[],["uuid"]]],[11,"parse_str","","Parses a `Uuid` from a string of hexadecimal digits with…",10,[[["str"]],[["result",["uuid","error"]],["uuid"],["error"]]]],[11,"to_hyphenated","","Get a [`Hyphenated`] formatter.",10,[[],["hyphenated"]]],[11,"to_hyphenated_ref","","Get a borrowed [`HyphenatedRef`] formatter.",10,[[["self"]],["hyphenatedref"]]],[11,"to_simple","","Get a [`Simple`] formatter.",10,[[],["simple"]]],[11,"to_simple_ref","","Get a borrowed [`SimpleRef`] formatter.",10,[[["self"]],["simpleref"]]],[11,"to_urn","","Get a [`Urn`] formatter.",10,[[],["urn"]]],[11,"to_urn_ref","","Get a borrowed [`UrnRef`] formatter.",10,[[["self"]],["urnref"]]],[11,"new_v1","","Create a new UUID (version 1) using a time value +…",10,[[["timestamp"]],[["result",["uuid","error"]],["uuid"],["error"]]]],[11,"to_timestamp","","Returns an optional [`Timestamp`] storing the timestamp…",10,[[["self"]],[["option",["timestamp"]],["timestamp"]]]],[11,"new_v3","","Creates a UUID using a name from a namespace, based on the…",10,[[["uuid"]],["uuid"]]],[11,"new_v4","","Creates a random UUID.",10,[[],["uuid"]]],[18,"NAMESPACE_DNS","","UUID namespace for Domain Name System (DNS).",10,null],[18,"NAMESPACE_OID","","UUID namespace for ISO Object Identifiers (OIDs).",10,null],[18,"NAMESPACE_URL","","UUID namespace for Uniform Resource Locators (URLs).",10,null],[18,"NAMESPACE_X500","","UUID namespace for X.500 Distinguished Names (DNs).",10,null],[11,"get_variant","","Returns the variant of the UUID structure.",10,[[["self"]],[["option",["variant"]],["variant"]]]],[11,"get_version_num","","Returns the version number of the UUID.",10,[[["self"]],["usize"]]],[11,"get_version","","Returns the version of the UUID.",10,[[["self"]],[["version"],["option",["version"]]]]],[11,"as_fields","","Returns the four field values of the UUID in big-endian…",10,[[["self"]]]],[11,"to_fields_le","","Returns the four field values of the UUID in little-endian…",10,[[["self"]]]],[11,"as_u128","","Returns a 128bit value containing the UUID data.",10,[[["self"]],["u128"]]],[11,"to_u128_le","","Returns a 128bit little-endian value containing the UUID…",10,[[["self"]],["u128"]]],[11,"as_bytes","","Returns an array of 16 octets containing the UUID data.",10,[[["self"]]]],[11,"is_nil","","Tests if the UUID is nil.",10,[[["self"]],["bool"]]],[11,"encode_buffer","","A buffer that can be used for `encode_...` calls, that is…",10,[[]]],[11,"new_with_scale","tiberius::numeric","Creates a new Numeric value.",11,[[["i128"],["u8"]],["self"]]],[11,"dec_part","","Extract the decimal part.",11,[[],["i128"]]],[11,"int_part","","Extract the integer part.",11,[[],["i128"]]],[11,"scale","","The scale (where is the decimal point) of the value.",11,[[["self"]],["u8"]]],[11,"value","","The internal integer value",11,[[["self"]],["i128"]]],[11,"new","tiberius::time","Construct a new `DateTime` instance.",12,[[["u32"],["i32"]],["self"]]],[11,"days","","Days since 1st of January, 1900 (including the negative…",12,[[["self"]],["i32"]]],[11,"seconds_fragments","","1/300 of a second, so a value of 300 equals 1 second…",12,[[["self"]],["u32"]]],[11,"new","","Construct a new `SmallDateTime` instance.",13,[[["u16"]],["self"]]],[11,"days","","Days since 1st of January, 1900.",13,[[["self"]],["u16"]]],[11,"seconds_fragments","","1/300 of a second, so a value of 300 equals 1 second…",13,[[["self"]],["u16"]]],[11,"new","","Construct a new `Date`",14,[[["u32"]],["date"]]],[11,"days","","The number of days from 1st of January, year 1.",14,[[["self"]],["u32"]]],[11,"new","","Construct a new `Time`",15,[[["u64"],["u8"]],["self"]]],[11,"increments","","Number of 10^-n second increments since midnight, where…",15,[[["self"]],["u64"]]],[11,"scale","","The accuracy of the increments.",15,[[["self"]],["u8"]]],[11,"new","","Construct a new `DateTime2` from the date and time…",16,[[["time"],["date"]],["self"]]],[11,"date","","The date component.",16,[[["self"]],["date"]]],[11,"time","","The time component.",16,[[["self"]],["time"]]],[11,"new","","Construct a new `DateTimeOffset` from a `datetime2`,…",17,[[["i16"],["datetime2"]],["self"]]],[11,"datetime2","","The date and time part.",17,[[["self"]],["datetime2"]]],[11,"offset","","Number of minutes from UTC.",17,[[["self"]],["i16"]]],[11,"db_name","tiberius::xml","Specifies the name of the database where the schema…",18,[[["self"]],["str"]]],[11,"owner","","Specifies the name of the relational schema containing the…",18,[[["self"]],["str"]]],[11,"collection","","Specifies the name of the XML schema collection to which…",18,[[["self"]],["str"]]],[11,"new","","Create a new XmlData with the given string. Validation of…",19,[[],["self"]]],[11,"schema","","Returns information about the schema of the XML file, if…",19,[[["self"]],[["option",["xmlschema"]],["xmlschema"]]]],[11,"into_string","","Takes the XML string out from the struct.",19,[[],["string"]]]],"p":[[4,"AuthMethod"],[4,"Error"],[4,"EncryptionLevel"],[3,"ClientBuilder"],[3,"Client"],[3,"QueryResult"],[3,"ExecuteResult"],[3,"Column"],[3,"Row"],[8,"ToSql"],[3,"Uuid"],[3,"Numeric"],[3,"DateTime"],[3,"SmallDateTime"],[3,"Date"],[3,"Time"],[3,"DateTime2"],[3,"DateTimeOffset"],[3,"XmlSchema"],[3,"XmlData"]]};
addSearchOptions(searchIndex);initSearch(searchIndex);