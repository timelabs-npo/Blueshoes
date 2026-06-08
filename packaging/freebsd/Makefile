# FreeBSD Port Makefile for bs-edge-agent

PORTNAME=	bs-edge-agent
DISTVERSION=	0.1.0
CATEGORIES=	sysutils

MAINTAINER=	operator@blueshoe.io
COMMENT=	Sovereign edge orchestration agent with UDS telemetry for FreeBSD runtimes

LICENSE=	MIT

USES=		cargo
PLIST_FILES=	bin/bs-edge-agent \
		etc/rc.d/bs_edge_agent

CARGO_CRATES=	\
		chacha20poly1305-0.10.1 \
		serde-1.0.197 \
		serde_json-1.0.114 \
		md5-0.7.0 \
		rand-0.8.5

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/target/release/bs-edge-agent ${STAGEDIR}${PREFIX}/bin
	${INSTALL_SCRIPT} ${WRKSRC}/packaging/freebsd/rc.d/bs_edge_agent ${STAGEDIR}${PREFIX}/etc/rc.d

.include <bsd.port.mk>
