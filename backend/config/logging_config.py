import logging

def get_logger(name):
    """
    Get a logger with the specified name
    
    Usage:
        from config.logging_config import get_logger
        logger = get_logger(__name__)
        logger.info("Something happened")
    """
    return logging.getLogger(name)

def log_request(logger, request, message="Request received"):
    """
    Log HTTP request details
    
    Usage:
        log_request(logger, request, "Heartbeat received")
    """
    logger.info(f"{message}")
    logger.debug(f"Method: {request.method}")
    logger.debug(f"Path: {request.path}")
    logger.debug(f"Remote IP: {request.META.get('REMOTE_ADDR', 'unknown')}")
    logger.debug(f"User Agent: {request.META.get('HTTP_USER_AGENT', 'unknown')}")

def log_response(logger, response, message="Response sent"):
    """
    Log HTTP response details
    
    Usage:
        log_response(logger, response, "Heartbeat response")
    """
    logger.info(f"{message}")
    logger.debug(f"Status: {response.status_code}")
    logger.debug(f"Content-Type: {response.get('Content-Type', 'unknown')}")