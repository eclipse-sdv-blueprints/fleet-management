#[cfg(test)]

tonic::include_proto!("kuksa.val.v1");

 
use self::datapoint::Value;
use crate::vehicle_abstraction::kuksa;

use protobuf::{MessageField, well_known_types::timestamp::Timestamp};

use fms_proto::fms::VehicleStatus;
use fms_proto::fms::KeyValue;
use fms_proto::fms::SnapshotData;

mod kuksa_tests {
    #[test]
    fn test_new_vehicle_status() {
        let mut my_map: HashMap<String, Value> = HashMap::new();

        // Insert key-value pairs into the HashMap
        my_map.insert("name".to_string(), Value::String("John".to_string()));
        my_map.insert("age".to_string(), Value::Number(30.into()));
        my_map.insert("is_student".to_string(), Value::Bool(false));

        let actual = vehicle_abstraction::kuksa::new_vehicle_status(my_map, "123");
        let mut expected = VehicleStatus::new();

        expected.created = MessageField::some(Timestamp::now());

        let mut snapshot_data_vec = SnapshotData::new();
        expected.snapshot_data = MessageField::some(snapshot_data_vec);
    
        assert_eq!(actual, expected);
        // assert_eq!(2 + 2, 4);
    }

   // #[test]
  /*   fn test_aasubtraction() {
        assert_eq!(5 - 3, 1);
    }*/

    // Add more tests as needed
}