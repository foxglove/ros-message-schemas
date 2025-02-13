from typing import List, Optional, Tuple

class MCAPWriter:
    def __new__(cls) -> "MCAPWriter": ...
    def close(self) -> None: ...

class WebSocketServer:
    """
    A websocket server for live visualization.
    """

    def __new__(cls) -> "WebSocketServer": ...
    def stop(self) -> None: ...
    def clear_session(self, session_id: Optional[str] = None) -> None: ...

class BaseChannel:
    """
    A channel for logging messages.
    """

    def __new__(
        cls,
        topic: str,
        message_encoding: str,
        schema_name: Optional[str] = None,
        schema_encoding: Optional[str] = None,
        schema_data: Optional[bytes] = None,
        metadata: Optional[List[Tuple[str, str]]] = None,
    ) -> "BaseChannel": ...
    def log(
        self,
        msg: bytes,
        publish_time: Optional[int] = None,
        log_time: Optional[int] = None,
        sequence: Optional[int] = None,
    ) -> None: ...

class PartialMetadata:
    """
    Structured metadata for use with logging. All fields are optional.
    """

    def __new__(
        cls,
        sequence: Optional[int] = None,
        log_time: Optional[int] = None,
        publish_time: Optional[int] = None,
    ) -> "PartialMetadata":
        """
        :param sequence: The sequence number is unique per channel and allows for ordering of
            messages as well as detecting missing messages. If omitted, a monotonically increasing
            sequence number unique to the channel is used.
        :param log_time: The log time is the time, as nanoseconds from the unix epoch, that the
            message was recorded. Usually this is the time log() is called. If omitted, the
            current time is used.
        :param publish_time: The publish_time is the time at which the message was published. e.g.
            the timestamp at which the sensor reading was taken. If omitted, log time is used.
        """
        ...

def start_server(
    name: Optional[str] = None,
    host: Optional[str] = "127.0.0.1",
    port: Optional[int] = 0,
) -> WebSocketServer:
    """
    Start a websocket server for live visualization.
    """
    ...

def enable_log_forwarding(level: str) -> None:
    """
    Forward SDK logs to python's logging facility.
    """
    ...

def disable_log_forwarding() -> None:
    """
    Stop forwarding SDK logs.
    """
    ...

def shutdown() -> None:
    """
    Shutdown the running websocket server.
    """
    ...

def record_file(path: str) -> None:
    """
    Create a new MCAP file at ``path`` for logging.
    """
    ...

def get_channel_for_topic(topic: str) -> BaseChannel:
    """
    Get a previously-registered channel.
    """
    ...
