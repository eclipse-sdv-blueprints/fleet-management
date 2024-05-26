// SPDX-FileCopyrightText: 2023 Contributors to the Eclipse Foundation
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

use chrono::{DateTime, Timelike, Utc};
use geotab_curve::{self, Curve, Sample};
use std::collections::VecDeque;
use std::fmt;
use tokio::sync::{mpsc, oneshot};
pub const POSITIONAL_ALLOWED_ERROR: f32 = 20.0; // in meters
pub const AMOUNT_OF_SIGNALS: usize = 3;
pub const SCALAR_ALLOWED_ERROR: f32 = 7.0;
pub const WINDOW_CAPACITY: usize = 10;
pub const CAP_VALUE: usize = WINDOW_CAPACITY;

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

impl geotab_curve::Valid for PositionSample {
    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl geotab_curve::Save for PositionSample {
    fn is_save(&self) -> bool {
        self.save
    }

    fn set_save(&mut self, save: bool) {
        self.save = save;
    }
}

impl geotab_curve::Position<f32> for PositionSample {
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
    pub was_written_to_influx: bool,
}

impl ScalarSample {
    pub fn new(time: DateTime<Utc>, value: f32) -> Self {
        Self {
            time,
            value,
            save: false,
            valid: true,
            was_written_to_influx: false,
        }
    }
}

impl fmt::Debug for ScalarSample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Scalar Data {{ time: {} minutes and {} seconds, value: {} }}\n",
            self.time.minute(),
            self.time.second(),
            self.value
        )
    }
}

impl geotab_curve::Sample for ScalarSample {
    fn time(&self) -> DateTime<Utc> {
        self.time
    }

    fn set_time(&mut self, time: DateTime<Utc>) {
        self.time = time;
    }
}

impl geotab_curve::Valid for ScalarSample {
    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl geotab_curve::Save for ScalarSample {
    fn is_save(&self) -> bool {
        self.save
    }

    fn set_save(&mut self, save: bool) {
        self.save = save;
    }
}

impl geotab_curve::ScalarValue<f32> for ScalarSample {
    fn value(&self) -> f32 {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct ChosenSignals {
    pub capacity: u32,
    pub speed: Option<f32>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub time: Option<i64>,
}

#[allow(dead_code)]
impl ChosenSignals {
    pub fn new() -> ChosenSignals {
        let capacity = 0;
        let speed: Option<f32> = None;
        let lat: Option<f64> = None;
        let lon: Option<f64> = None;
        let time: Option<i64> = None;
        ChosenSignals {
            capacity,
            speed,
            lat,
            lon,
            time,
        }
    }

    pub fn add_speed(&mut self, value: f32) {
        log::info!("\n\tAdding speed signal");
        self.speed = Some(value);
        self.capacity += 1;
    }

    pub fn add_lat(&mut self, value: f64) {
        log::info!("\n\tAdding latitude signal");
        self.lat = Some(value);
        self.capacity += 1;
    }

    pub fn add_lon(&mut self, value: f64) {
        log::info!("\n\tAdding longitude signal");
        self.lon = Some(value);
        self.capacity += 1;
    }

    pub fn add_time(&mut self, value: i64) {
        log::info!("\n\tAdding Time signal");
        self.time = Some(value);
    }

    pub fn is_full(&self) -> bool {
        let max_signal_quantity = AMOUNT_OF_SIGNALS;
        if self.capacity == max_signal_quantity as u32 {
            log::info!("\n\tAll relevant signals collected");
            true
        } else {
            log::error!("\tAll relevant signals could not be collected collected");
            false
        }
    }
}

#[derive(Debug)]
struct CurveLogActor {
    is_reduced: bool,
    time_tracker: i64,
    receiver: mpsc::Receiver<ActorMessage>,
    speed_dps: Vec<f32>,
    lat_dps: Vec<f64>,
    lon_dps: Vec<f64>,
    time_dps: VecDeque<i64>,
}
enum ActorMessage {
    GetCurvedResult {
        respond_to: oneshot::Sender<Option<Vec<ChosenSignals>>>,
    },
    SendData {
        data: ChosenSignals,
    },
}

impl CurveLogActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>, _vin: String) -> Self {
        let is_reduced = false;
        let time_tracker = 0;
        let speed_dps = Vec::with_capacity(WINDOW_CAPACITY);
        let lon_dps = Vec::with_capacity(WINDOW_CAPACITY);
        let lat_dps = Vec::with_capacity(WINDOW_CAPACITY);
        let time_dps = VecDeque::new();
        CurveLogActor {
            is_reduced,
            speed_dps,
            lon_dps,
            lat_dps,
            time_dps,
            receiver,
            time_tracker,
        }
    }

    async fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetCurvedResult { respond_to } => {
                if self.is_reduced == true {
                    log::info!("Repackaging of result back into an SLLT...");
                    let mut ret: Vec<ChosenSignals> = Vec::new();
                    for i in 0..WINDOW_CAPACITY - 1 {
                        let mut sllt = ChosenSignals::new();
                        if let Some(speed) = self.speed_dps.get(i) {
                            sllt.add_speed(*speed);
                            self.speed_dps.pop();
                        }
                        if let Some(latitude) = self.lat_dps.get(i) {
                            sllt.add_lat(*latitude);
                            self.lat_dps.pop();
                        }
                        if let Some(longitude) = self.lon_dps.get(i) {
                            sllt.add_lon(*longitude);
                            self.lon_dps.pop();
                        }
                        if let Some(time) = self.time_dps.get(i) {
                            sllt.add_time(*time);
                            self.time_dps.pop_front();
                        } else {
                            break;
                        }
                        ret.push(sllt);
                    }
                    self.is_reduced = false;
                    let _ = respond_to.send(Some(ret));
                } else {
                    let _ = respond_to.send(None);
                }
            }
            ActorMessage::SendData { data } => {
                log::info!("Chosen Signals: {:?}", data);
                self.speed_dps.push(data.speed.unwrap().clone());
                self.lon_dps.push(data.lon.unwrap().clone());
                self.lat_dps.push(data.lat.unwrap().clone());
                self.time_dps.push_back(self.time_tracker);
                self.time_tracker += 1;
                log::info!("\t\tChecking for curvelogging...");
                if self.speed_dps.len() == WINDOW_CAPACITY {
                    log::info!("\t\tReady to curve");
                    let reduced_scalar_dps =
                        process_speed_window(&mut self.speed_dps, &mut self.time_dps.clone()).await;
                    let reduced_positional_dps = process_lon_lat_window(
                        &mut self.lon_dps,
                        &mut self.lat_dps,
                        &mut self.time_dps.clone(),
                    )
                    .await;
                    self.is_reduced = true;
                    self.speed_dps.clear();
                    self.lon_dps.clear();
                    self.lat_dps.clear();
                    self.time_dps.clear();
                    self.speed_dps = reduced_scalar_dps.iter().map(|dps| dps.value).collect();
                    self.time_dps = reduced_scalar_dps
                        .iter()
                        .map(|dps| dps.time.timestamp())
                        .collect();
                    self.lon_dps = reduced_positional_dps
                        .iter()
                        .map(|dps| dps.lon as f64)
                        .collect();
                    self.lat_dps = reduced_positional_dps
                        .iter()
                        .map(|dps| dps.lat as f64)
                        .collect();
                }
                log::info!("\n\n\tNot enough elements: {}\n\n\n", self.speed_dps.len());
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
pub struct CurveLogActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl CurveLogActorHandle {
    pub fn new(vin: String) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = CurveLogActor::new(receiver, vin);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub async fn get_curved_results(&self) -> Option<Vec<ChosenSignals>> {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetCurvedResult { respond_to: send };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    pub async fn send_data(&self, sllt: ChosenSignals) {
        log::info!("Sending SLLT (Speed, Latitude, Longitude and Timestamp)...");
        let msg = ActorMessage::SendData { data: sllt };
        let _ = self.sender.send(msg).await;
    }
}

pub async fn process_speed_window(
    speed_dps: &mut Vec<f32>,
    speed_times: &mut VecDeque<i64>,
) -> Vec<ScalarSample> {
    log::info!("Starting Scalar and Positional Curlog\n");
    pub type SampleCurve = geotab_curve::ScalarValueCurve<ScalarSample, f32, { CAP_VALUE }>;

    let mut curve = SampleCurve::new();
    let mut saved: Vec<ScalarSample> = Vec::new();

    //adding first element manually
    let first_element = ScalarSample::new(
        DateTime::from_timestamp(speed_times.get_mut(0).unwrap().to_owned(), 0).unwrap(),
        speed_dps.get_mut(0).unwrap().to_owned(),
    );

    for speed in speed_dps.into_iter() {
        let time_to_convert = speed_times.pop_front().unwrap().to_owned();
        let time = DateTime::from_timestamp(time_to_convert, 0).unwrap();
        let sample = ScalarSample::new(time, speed.to_owned());
        curve.add_value(sample);
        if curve.is_full() {
            let reduced: geotab_curve::CurveSaveIter<
                '_,
                ScalarSample,
                geotab_curve::ScalarValueCurve<ScalarSample, f32, { CAP_VALUE }>,
            > = curve.reduce(SCALAR_ALLOWED_ERROR, true, true).unwrap();
            saved.extend(reduced);
            log::info!("Reduced curve");
        }
    }
    let reduced = curve.reduce(SCALAR_ALLOWED_ERROR, false, true);
    match reduced {
        Some(value) => {
            saved.extend(value);
        }
        None => {
            // std::process::exit(1);
        }
    }
    saved.insert(0, first_element);
    saved
}

pub async fn process_lon_lat_window(
    longitude_dps: &mut Vec<f64>,
    latitude_dps: &mut Vec<f64>,
    speed_times: &mut VecDeque<i64>,
) -> Vec<PositionSample> {
    type SampleCurve = geotab_curve::PositionCurve<PositionSample, f32, 5>;
    let mut curve = SampleCurve::new();
    let mut saved = vec![];
    log::info!(
        "\t\tStarting curvelogging of positional signals: \nLON {:#?} \nLAT{:#?}, \nTIMES: {:#?}",
        longitude_dps,
        latitude_dps,
        speed_times
    );

    //handle first element manually
    let datetime = DateTime::from_timestamp(speed_times.get_mut(0).unwrap().to_owned(), 0).unwrap();
    let first_element = PositionSample::new(
        datetime,
        latitude_dps.get_mut(0).unwrap().to_owned() as f32,
        longitude_dps.get_mut(0).unwrap().to_owned() as f32,
    );

    for _i in 0..WINDOW_CAPACITY {
        let lat = latitude_dps.pop().unwrap().to_owned() as f32;
        let lon = longitude_dps.pop().unwrap().to_owned() as f32;
        let time = DateTime::from_timestamp(speed_times.pop_front().unwrap(), 0).unwrap();
        let sample = PositionSample::new(time, lat, lon);
        curve.add_value(sample);
        // Reduce the curve buffer when it gets full. Don't force save last point, but make sure to
        // populate the next curve with unsaved curves from the reduced curve (run_on = true)
        if curve.is_full() {
            let reduced = curve.reduce(POSITIONAL_ALLOWED_ERROR, true, true).unwrap();
            saved.extend(reduced);
        }
    }
    let reduced = curve.reduce(POSITIONAL_ALLOWED_ERROR, false, true);
    match reduced {
        Some(value) => {
            saved.extend(value);
        }
        None => {
            // std::process::exit(1);
        }
    }
    saved.insert(0, first_element);
    saved
}
