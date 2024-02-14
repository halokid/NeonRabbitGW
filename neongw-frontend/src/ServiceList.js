
import React from 'react';

const serviceData = [
  { name: 'Service 1', address: 'Address 1', status: 'Active' },
  { name: 'Service 2', address: 'Address 2', status: 'Inactive' },
  { name: 'Service 3', address: 'Address 3', status: 'Active' },
];

const ServiceList = () => {
  return (
    <div className="container mt-4">
      <h2 className="mb-4">Service List</h2>
      <div className="table-responsive">
        <table className="table table-bordered table-striped" style={{ width: '100%' }}>
          <thead>
          <tr>
            <th scope="col">#</th>
            <th scope="col">Name</th>
            <th scope="col">Address</th>
            <th scope="col">Status</th>
          </tr>
          </thead>
          <tbody>
          {serviceData.map((service, index) => (
            <tr key={index}>
              <th scope="row">{index + 1}</th>
              <td>{service.name}</td>
              <td>{service.address}</td>
              <td>{service.status}</td>
            </tr>
          ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default ServiceList;

