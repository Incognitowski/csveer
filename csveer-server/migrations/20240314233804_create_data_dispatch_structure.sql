CREATE TABLE data_dispatch(
  id SERIAL PRIMARY KEY,
  file_destination_id INT NOT NULL,
  status VARCHAR(100) NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ,
  FOREIGN KEY(file_destination_id) REFERENCES file_destination(id)
);

CREATE TABLE data_dispatch_execution(
  id SERIAL PRIMARY KEY,
  data_dispatch_id INT NOT NULL,
  status VARCHAR(100) NOT NULL,
  message TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  FOREIGN KEY (data_dispatch_id) REFERENCES data_dispatch(id)
);
