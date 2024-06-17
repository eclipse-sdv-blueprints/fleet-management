// SPDX-FileCopyrightText: 2024 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Duration, FixedOffset, TimeZone, Timelike, Utc};
use geotab_curve::{
    self, Curve, Position, PositionCurve, Sample, Save, ScalarValue, ScalarValueCurve, Valid,
};
use std::collections::HashSet;

use std::collections::VecDeque;
use std::fmt;
use tokio::sync::mpsc;

pub const POSITIONAL_ALLOWED_ERROR: f32 = 20.0; // in meters
pub const POSITIONAL_CAP: usize = 5;
pub const SCALAR_ALLOWED_ERROR: f32 = 7.0;
pub const SCALAR_CAP: usize = 7;
pub const PARAM_WINDOW_CAPACITY: &str = "window-capacity";

#[derive(Debug, Clone)]
pub struct PositionSample {
    pub time: DateTime<Utc>,
    pub lat: f32,
    pub lon: f32,
    pub save: bool,
    pub valid: bool,
}

impl PositionSample {
    pub fn new(time: DateTime<Utc>, lat: f32, lon: f32) -> Self {
        Self {
            time,
            lat,
            lon,
            save: false,
            valid: true,
        }
    }
}

impl Sample for PositionSample {
    fn time(&self) -> DateTime<Utc> {
        self.time
    }

    fn set_time(&mut self, time: DateTime<Utc>) {
        self.time = time;
    }
}

impl Valid for PositionSample {
    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl Save for PositionSample {
    fn is_save(&self) -> bool {
        self.save
    }

    fn set_save(&mut self, save: bool) {
        self.save = save;
    }
}

impl Position<f32> for PositionSample {
    fn latitude(&self) -> f32 {
        self.lat
    }

    fn longitude(&self) -> f32 {
        self.lon
    }
}

#[derive(Clone)]
pub struct ScalarSample {
    pub time: DateTime<Utc>,
    pub value: f32,
    pub save: bool,
    pub valid: bool,
}

impl ScalarSample {
    pub fn new(time: DateTime<Utc>, value: f32) -> Self {
        Self {
            time,
            value,
            save: false,
            valid: true,
        }
    }
}

impl fmt::Debug for ScalarSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Scalar Data {{ time: {} minutes and {} seconds, value: {} }}\n",
            self.time.minute(),
            self.time.second(),
            self.value
        )
    }
}

impl Sample for ScalarSample {
    fn time(&self) -> DateTime<Utc> {
        self.time
    }

    fn set_time(&mut self, time: DateTime<Utc>) {
        self.time = time;
    }
}

impl Valid for ScalarSample {
    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl Save for ScalarSample {
    fn is_save(&self) -> bool {
        self.save
    }

    fn set_save(&mut self, save: bool) {
        self.save = save;
    }
}

impl ScalarValue<f32> for ScalarSample {
    fn value(&self) -> f32 {
        self.value
    }
}

pub struct Signal {
    pub speed: f32,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub time: u128,
}

#[derive(Debug, Clone)]
pub struct ChosenSignals {
    pub speed: Option<f32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub time: u128,
}

#[allow(dead_code)]
impl ChosenSignals {
    pub fn new() -> ChosenSignals {
        let speed: Option<f32> = None;
        let lat: Option<f64> = None;
        let lon: Option<f64> = None;
        let time: u128 = 0;
        ChosenSignals {
            speed,
            lat,
            lon,
            time,
        }
    }

    pub fn add_speed(&mut self, value: Option<f32>) {
        log::debug!("\n\tAdding speed signal");
        self.speed = value;
    }

    pub fn add_lat(&mut self, value: Option<f64>) {
        log::debug!("\n\tAdding latitude signal");
        self.lat = value;
    }

    pub fn add_lon(&mut self, value: Option<f64>) {
        log::debug!("\n\tAdding longitude signal");
        self.lon = value;
    }

    pub fn add_time(&mut self, value: u128) {
        log::debug!("\n\tAdding Time signal");
        self.time = value;
    }
}

#[derive(Debug)]
struct CurveLogActor {
    window_capacity: usize,
    receiver: mpsc::Receiver<ActorMessage>,
    speed_datapoints: Vec<Option<f32>>,
    lat_datapoints: Vec<Option<f64>>,
    lon_datapoints: Vec<Option<f64>>,
    time_speed_datapoints: VecDeque<Option<u128>>,
    publisher_sender: tokio::sync::mpsc::Sender<Vec<Option<ChosenSignals>>>,
}
enum ActorMessage {
    SendSignals { signal: Signal },
    // SendLongitudeAndLatitude { position: PositionSignal },
}

impl CurveLogActor {
    fn new(
        receiver: mpsc::Receiver<ActorMessage>,
        window_capacity: usize,
        publisher_sender: tokio::sync::mpsc::Sender<Vec<Option<ChosenSignals>>>,
    ) -> Self {
        let speed_datapoints = Vec::with_capacity(window_capacity);
        let lon_datapoints = Vec::with_capacity(window_capacity);
        let lat_datapoints = Vec::with_capacity(window_capacity);
        let time_speed_datapoints = VecDeque::new();
        CurveLogActor {
            window_capacity,
            speed_datapoints,
            lon_datapoints,
            lat_datapoints,
            time_speed_datapoints,
            receiver,
            publisher_sender,
        }
    }

    async fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::SendSignals { signal } => {
                let mut reduced_speed_collection: Vec<Option<ChosenSignals>> = Vec::new();
                self.speed_datapoints.push(Some(signal.speed));
                self.lon_datapoints.push(signal.lon);
                self.lat_datapoints.push(signal.lat);
                self.time_speed_datapoints.push_back(Some(signal.time));
                if self.speed_datapoints.len() == self.window_capacity {
                    log::info!("Enough elements to Scalar Curvelog!");
                    println!(
                        "Curving scalarly: {:?}\n Len: {}\n Times: {:?}\n Len: {}\n LON: {:?}\n LAT{:?}",
                        self.speed_datapoints,
                        self.speed_datapoints.len(),
                        self.time_speed_datapoints,
                        self.time_speed_datapoints.len(),
                        self.lon_datapoints,
                        self.lat_datapoints,
                    );
                    let original_time_scalar_datapoints_len = &self.time_speed_datapoints.len();
                    let reduced_scalar_datapoints = process_speed_window(
                        &self.speed_datapoints,
                        &self.time_speed_datapoints.clone(),
                    )
                    .await;
                    let reduced_position_datapoints = process_lon_lat_window(
                        &self.lon_datapoints,
                        &self.lat_datapoints,
                        &self.time_speed_datapoints.clone(),
                    )
                    .await;
                    println!(
                        "Curved Scalar result: {:?}\n Len: {}\n",
                        reduced_scalar_datapoints,
                        reduced_scalar_datapoints.len(),
                    );
                    println!(
                        "Curved Position result: {:?}\n Len: {}\n",
                        reduced_position_datapoints,
                        reduced_position_datapoints.len(),
                    );
                    repackage_scalar(
                        &mut self.speed_datapoints,
                        &mut self.lon_datapoints,
                        &mut self.lat_datapoints,
                        &mut self.time_speed_datapoints,
                        reduced_scalar_datapoints.clone(),
                        reduced_position_datapoints.clone(),
                    );
                    let reduction_percentage = get_reduction_percentage(
                        count_some(&self.speed_datapoints),
                        original_time_scalar_datapoints_len.to_owned(),
                    );
                    println!(
                        "Curved Scalar: {:?}, Len: {}\n Reduction percentage: {}\n, Time: {:?}, Len {}\n, Lon: {:?}\n, Lat: {:?}\n, POSITION LEN: {:?}\n",
                        self.speed_datapoints,
                        self.speed_datapoints.len(),
                        reduction_percentage,
                        self.time_speed_datapoints,
                        self.time_speed_datapoints.len(),
                        self.lon_datapoints,
                        self.lat_datapoints,
                        self.lat_datapoints.len(),
                    );
                    log::debug!("Sending for publishing...");
                    let mut counter = 0;
                    //register all the survived values and keep last for next iteration
                    while counter != self.time_speed_datapoints.len() - 1 {
                        let mut sllt = ChosenSignals::new();
                        if let Some(time) = self.time_speed_datapoints.get(counter).unwrap() {
                            sllt.add_time(time.to_owned());
                            if let Some(speed) = self.speed_datapoints.get(counter).unwrap() {
                                sllt.add_speed(Some(speed.to_owned()));
                            };
                            if let Some(lon) = self.lon_datapoints.get(counter).unwrap() {
                                sllt.add_lon(Some(lon.to_owned()));
                            };
                            if let Some(lat) = self.lat_datapoints.get(counter).unwrap() {
                                sllt.add_lat(Some(lat.to_owned()));
                            };
                            reduced_speed_collection.push(Some(sllt));
                        } else {
                            reduced_speed_collection.push(None);
                        }
                        counter += 1;
                    }
                    drain_all_elements_but_last(
                        &mut self.speed_datapoints,
                        &mut self.time_speed_datapoints,
                        &mut self.lon_datapoints,
                        &mut self.lat_datapoints,
                    );
                    match self.publisher_sender.send(reduced_speed_collection).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::warn!("failed to send curvelogged speed via channel: {}", e);
                        }
                    }
                }
            }
        }
    }
}

async fn run_my_actor(mut actor: CurveLogActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

#[derive(Clone, Debug)]
pub struct CurveLogActorHandler {
    sender: mpsc::Sender<ActorMessage>,
}

impl CurveLogActorHandler {
    pub fn new(
        window_capacity: usize,
        publisher_sender: tokio::sync::mpsc::Sender<Vec<Option<ChosenSignals>>>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = CurveLogActor::new(receiver, window_capacity, publisher_sender);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub async fn send_signals(
        &self,
        speed: Option<f32>,
        lon: Option<f64>,
        lat: Option<f64>,
        time: u128,
    ) {
        if let Some(speed) = speed {
            log::debug!(
                "Getting new Speed km/h signal: {} at position LON: {:?} LAT: {:?} at {}",
                speed,
                lon,
                lat,
                convert_to_current_time(time)
            );
            let signal = Signal {
                speed,
                lon,
                lat,
                time,
            };
            let msg = ActorMessage::SendSignals { signal };
            let _ = self.sender.send(msg).await;
        }
    }
}

pub async fn process_speed_window(
    speed_datapoints: &Vec<Option<f32>>,
    speed_times: &VecDeque<Option<u128>>,
) -> Vec<ScalarSample> {
    pub type SampleCurve = ScalarValueCurve<ScalarSample, f32, SCALAR_CAP>;
    let mut speed_datapoints_copy = speed_datapoints.to_owned().clone();
    let mut speed_times_datapoints = speed_times.clone();

    let mut curve = SampleCurve::new();
    let mut saved = vec![];

    let time = speed_times_datapoints
        .get_mut(0)
        .unwrap()
        .to_owned()
        .unwrap() as u64;
    let datetime = convert_timestamp(time);
    //adding first element manually
    let first_element = ScalarSample::new(
        datetime,
        speed_datapoints_copy
            .get_mut(0)
            .unwrap()
            .to_owned()
            .unwrap(),
    );

    for (i, speed) in speed_datapoints_copy.into_iter().enumerate() {
        let time = DateTime::<Utc>::MIN_UTC + Duration::seconds(i as i64);

        let sample = ScalarSample::new(time, speed.to_owned().unwrap());
        curve.add_value(sample);
        if curve.is_full() {
            if let Some(reduced) = curve.reduce(SCALAR_ALLOWED_ERROR, true, false) {
                saved.extend(reduced);
                log::debug!("Reduced curve");
            }
        }
    }
    let reduced = curve.reduce(SCALAR_ALLOWED_ERROR, false, true).unwrap();
    saved.extend(reduced);
    saved.insert(0, first_element);
    saved
}

pub async fn process_lon_lat_window(
    longitude_datapoints: &Vec<Option<f64>>,
    latitude_datapoints: &Vec<Option<f64>>,
    time_position_datapoints: &VecDeque<Option<u128>>,
) -> Vec<PositionSample> {
    type SampleCurve = PositionCurve<PositionSample, f32, POSITIONAL_CAP>;
    let mut lon_datapoints_copy = longitude_datapoints.to_owned().clone();
    let mut lat_datapoints_copy = latitude_datapoints.to_owned().clone();
    let mut position_times_datapoints = time_position_datapoints.clone();

    let mut curve = SampleCurve::new();
    let mut saved = vec![];

    let time = position_times_datapoints
        .get_mut(0)
        .unwrap()
        .to_owned()
        .unwrap() as u64;
    let datetime = convert_timestamp(time);
    let first_element = PositionSample::new(
        datetime,
        lat_datapoints_copy.get_mut(0).unwrap().to_owned().unwrap() as f32,
        lon_datapoints_copy.get_mut(0).unwrap().to_owned().unwrap() as f32,
    );

    for (i, lat) in lat_datapoints_copy.iter_mut().enumerate() {
        let lat = lat.to_owned().unwrap() as f32;
        let lon = lon_datapoints_copy.get_mut(i).unwrap().to_owned().unwrap() as f32;
        let time = DateTime::<Utc>::MIN_UTC + Duration::seconds(i as i64);
        let sample = PositionSample::new(time, lat, lon);
        curve.add_value(sample);
        // Reduce the curve buffer when it gets full. Don't force save last point, but make sure to
        // populate the next curve with unsaved curves from the reduced curve (run_on = true)
        if curve.is_full() {
            let reduced = curve.reduce(POSITIONAL_ALLOWED_ERROR, true, false).unwrap();
            saved.extend(reduced);
        }
    }
    let reduced = curve.reduce(POSITIONAL_ALLOWED_ERROR, false, true).unwrap();
    saved.extend(reduced);
    saved.insert(0, first_element);
    saved
}

pub fn convert_timestamp(timestamp: u64) -> DateTime<Utc> {
    if timestamp > 1_000_000_000_000 {
        // Convert microseconds to seconds and nanoseconds
        let secs = (timestamp / 1_000_000) as i64;
        let nanos = ((timestamp % 1_000_000) * 1_000) as u32;
        Utc.timestamp_opt(secs, nanos).unwrap()
    } else {
        // Convert seconds directly
        let secs = timestamp as i64;
        Utc.timestamp_opt(secs, 0).unwrap()
    }
}

pub fn convert_to_current_time(timestamp: u128) -> DateTime<FixedOffset> {
    if timestamp > 1_000_000_000_000 {
        let secs = (timestamp / 1_000_000) as i64;
        let nanos = ((timestamp % 1_000_000) * 1_000) as u32;
        Utc.timestamp_opt(secs, nanos)
            .unwrap()
            .with_timezone(&chrono::FixedOffset::east_opt(3600 * 2).unwrap())
    } else {
        // Convert seconds directly
        let secs = timestamp as i64;
        Utc.timestamp_opt(secs, 0)
            .unwrap()
            .with_timezone(&chrono::FixedOffset::east_opt(3600 * 2).unwrap())
    }
}

pub fn get_reduction_percentage(
    number_of_elements_after_reduction: usize,
    number_of_elements_before_reduction: usize,
) -> u64 {
    if number_of_elements_before_reduction == 0 {
        return 0;
    }

    let after = number_of_elements_after_reduction as u64;
    let before = number_of_elements_before_reduction as u64;

    if after >= before {
        // If there is no reduction or the count increased, return 0
        return 0;
    }

    let reduction = before - after;
    (reduction * 100) / before
}

pub fn repackage_scalar(
    original_speed_dps: &mut Vec<Option<f32>>,
    original_lon_dps: &mut Vec<Option<f64>>,
    original_lat_dps: &mut Vec<Option<f64>>,
    original_time_dps: &mut VecDeque<Option<u128>>,
    survived_scalar: Vec<ScalarSample>,
    survived_position: Vec<PositionSample>,
) {
    println!("repackaging...");
    // extract indexes of the survived scalar values
    let mut matching_scalar_indexes: Vec<usize> = survived_scalar
        .iter()
        .map(|x| x.time.second() as usize)
        .collect();
    let mut matching_position_indexes: Vec<usize> = survived_position
        .iter()
        .map(|x| x.time.second() as usize)
        .collect();
    matching_scalar_indexes[0] = 0;
    matching_position_indexes[0] = 0;
    let matching_time_indexes = merge_indexes(&matching_scalar_indexes, &matching_position_indexes);
    // matching_time_indexes[0] = 0;
    println!("Matching scalar indexes: {:?}", matching_scalar_indexes);
    println!("Matching position indexes: {:?}", matching_position_indexes);
    println!("Matching time indexes: {:?}", matching_time_indexes);
    println!("Speeds before: \n{:?}\n", original_speed_dps);
    println!(
        "Position before: \nLON{:?}\nLAT: {:?}\n",
        original_lon_dps, original_lat_dps
    );
    //set all the speed values that are not in the matching indexes to None
    for (i, speed) in original_speed_dps.iter_mut().enumerate() {
        if !matching_scalar_indexes.contains(&i) {
            *speed = None;
        }
    }
    for (i, lon) in original_lon_dps.iter_mut().enumerate() {
        if !matching_position_indexes.contains(&i) {
            *lon = None;
        }
    }
    for (i, lat) in original_lat_dps.iter_mut().enumerate() {
        if !matching_position_indexes.contains(&i) {
            *lat = None;
        }
    }
    for (i, time) in original_time_dps.iter_mut().enumerate() {
        if !matching_time_indexes.contains(&i) {
            *time = None;
        }
    }
    println!("Speeds: {:?}", original_speed_dps);
    println!("Lon: {:?}", original_lon_dps);
    println!("Lat: {:?}", original_lat_dps);
    println!("TIME: {:?}", original_time_dps);
}

pub fn drain_all_elements_but_last(
    speed_datapoints: &mut Vec<Option<f32>>,
    time_speed_datapoints: &mut VecDeque<Option<u128>>,
    lon_datapoints: &mut Vec<Option<f64>>,
    lat_datapoints: &mut Vec<Option<f64>>,
) {
    let last_speed = speed_datapoints.pop().unwrap();
    let last_time = time_speed_datapoints.pop_back().unwrap();
    let last_lon = lon_datapoints.pop().unwrap();
    let last_lat = lat_datapoints.pop().unwrap();
    speed_datapoints.clear();
    time_speed_datapoints.clear();
    lon_datapoints.clear();
    lat_datapoints.clear();
    speed_datapoints.push(last_speed);
    time_speed_datapoints.push_back(last_time);
    lon_datapoints.push(last_lon);
    lat_datapoints.push(last_lat);
}

fn count_some<T>(vec: &[Option<T>]) -> usize {
    vec.iter().filter(|x| x.is_some()).count()
}

fn merge_indexes(indexes1: &[usize], indexes2: &[usize]) -> Vec<usize> {
    let mut set: HashSet<usize> = HashSet::new();

    // Insert elements from the first array
    for &index in indexes1 {
        set.insert(index);
    }

    // Insert elements from the second array
    for &index in indexes2 {
        set.insert(index);
    }

    // Convert the HashSet to a Vec and sort it
    let mut merged_indexes: Vec<usize> = set.into_iter().collect();
    merged_indexes.sort();

    merged_indexes
}
