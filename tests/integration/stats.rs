use std::io::Write;

use serde_json;
use hyper::client::response::Response;
use hyper::http::h1::{Http11Message, HttpWriter};
use hyper::Url;

use dockworker::stats::{Stats, StatsStream};

use support::memory_stream::MemoryStream;

#[test]
fn single() {
  let response = get_stats_single_event(1);
  serde_json::from_str::<Stats>(&response).unwrap();
}

#[test]
fn streaming() {
  let url = Url::parse("http://localhost/stats").unwrap();
  let stream = MemoryStream::with_input(get_stats_response().as_bytes());
  let message = Http11Message::with_stream(Box::new(stream));
  let response = Response::with_message(url, Box::new(message)).unwrap();
  let mut reader = StatsStream::new(response);

  let stats = reader.next().unwrap().unwrap();
  assert_eq!(&stats.read, "2015-04-09T07:02:08.480022081Z");

  let stats = reader.next().unwrap().unwrap();
  assert_eq!(&stats.read, "2015-04-09T07:02:08.480022082Z");

  let stats = reader.next().unwrap().unwrap();
  assert_eq!(&stats.read, "2015-04-09T07:02:08.480022083Z");

  assert!(reader.next().is_none());
}

fn get_stats_response() -> String {
  let headers = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: Close\r\n\r\n";
  let s1 = get_stats_single_event(1);
  let s2 = get_stats_single_event(2);
  let s3 = get_stats_single_event(3);

  let stream = MemoryStream::with_input(headers.as_bytes());
  let mut writer = HttpWriter::ChunkedWriter(stream);
  writer.write(s1.as_bytes()).unwrap();
  writer.write(b"\n").unwrap();
  writer.write(s2.as_bytes()).unwrap();
  writer.write(b"\n").unwrap();
  writer.write(s3.as_bytes()).unwrap();

  let buf = match writer.end() {
    Ok(w) => w,
    Err(_) => {
      panic!("error ending writer for stats response");
    }
  };

  String::from_utf8(buf.into_inner()).unwrap()
}

fn get_stats_single_event(n: u64) -> String {
    format!("{{\"read\":\"2015-04-09T07:02:08.48002208{}Z\",\"network\":{{\"rx_bytes\":5820720,\"rx_packets\":2742,\"rx_errors\":0,\"rx_dropped\":1,\"tx_bytes\":158527,\"tx_packets\":2124,\"tx_errors\":0,\"tx_dropped\":0}},\"cpu_stats\":{{\"cpu_usage\":{{\"total_usage\":19194125000,\"percpu_usage\":[14110113138,3245604417,845722573,992684872],\"usage_in_kernelmode\":1110000000,\"usage_in_usermode\":18160000000}},\"system_cpu_usage\":1014488290000000,\"throttling_data\":{{\"periods\":0,\"throttled_periods\":0,\"throttled_time\":0}}}},\"memory_stats\":{{\"usage\":208437248,\"max_usage\":318791680,\"stats\":{{\"active_anon\":27213824,\"active_file\":129069056,\"cache\":178946048,\"hierarchical_memory_limit\":18446744073709551615,\"hierarchical_memsw_limit\":18446744073709551615,\"inactive_anon\":0,\"inactive_file\":49876992,\"mapped_file\":10809344,\"pgfault\":99588,\"pgmajfault\":819,\"pgpgin\":130731,\"pgpgout\":153466,\"rss\":29331456,\"rss_huge\":6291456,\"swap\":0,\"total_active_anon\":27213824,\"total_active_file\":129069056,\"total_cache\":178946048,\"total_inactive_anon\":0,\"total_inactive_file\":49876992,\"total_mapped_file\":10809344,\"total_pgfault\":99588,\"total_pgmajfault\":819,\"total_pgpgin\":130731,\"total_pgpgout\":153466,\"total_rss\":29331456,\"total_rss_huge\":6291456,\"total_swap\":0,\"total_unevictable\":0,\"total_writeback\":0,\"unevictable\":0,\"writeback\":0}},\"failcnt\":0,\"limit\":16854257664}},\"blkio_stats\":{{\"io_service_bytes_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":150687744}},{{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":150687744}},{{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":150687744}}],\"io_serviced_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":484}},{{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":484}},{{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":484}}],\"io_queue_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":0}}],\"io_service_time_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":2060941295}},{{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":2060941295}},{{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":2060941295}}],\"io_wait_time_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":5476872825}},{{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":5476872825}},{{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":5476872825}}],\"io_merged_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"Read\",\"value\":79}},{{\"major\":8,\"minor\":0,\"op\":\"Write\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Sync\",\"value\":0}},{{\"major\":8,\"minor\":0,\"op\":\"Async\",\"value\":79}},{{\"major\":8,\"minor\":0,\"op\":\"Total\",\"value\":79}}],\"io_time_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"\",\"value\":1814}}],\"sectors_recursive\":[{{\"major\":8,\"minor\":0,\"op\":\"\",\"value\":294312}}]}}}}", n).to_string()
}
