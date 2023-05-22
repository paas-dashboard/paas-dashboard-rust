FROM shoothzj/compile:rust AS build
COPY . /opt/compile
WORKDIR /opt/compile
RUN /root/.cargo/bin/cargo build --release

FROM shoothzj/base

WORKDIR /opt/paas-dashboard

COPY --from=build /opt/compile/target/release/paas-dashboard /opt/paas-dashboard/paas-dashboard

RUN wget -q https://github.com/paas-dashboard/paas-dashboard-portal-nextjs/releases/download/latest/paas-dashboard-portal.tar.gz && \
    tar -xzf paas-dashboard-portal.tar.gz && \
    rm -rf paas-dashboard-portal.tar.gz

EXPOSE 11111

CMD ["/usr/bin/dumb-init", "/opt/paas-dashboard/paas-dashboard"]
