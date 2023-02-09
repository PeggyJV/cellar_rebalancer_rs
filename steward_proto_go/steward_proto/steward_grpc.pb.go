// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.21.12
// source: steward.proto

package steward_proto

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// ContractCallClient is the client API for ContractCall service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type ContractCallClient interface {
	// Handles scheduled contract call submission
	Schedule(ctx context.Context, in *ScheduleRequest, opts ...grpc.CallOption) (*ScheduleResponse, error)
}

type contractCallClient struct {
	cc grpc.ClientConnInterface
}

func NewContractCallClient(cc grpc.ClientConnInterface) ContractCallClient {
	return &contractCallClient{cc}
}

func (c *contractCallClient) Schedule(ctx context.Context, in *ScheduleRequest, opts ...grpc.CallOption) (*ScheduleResponse, error) {
	out := new(ScheduleResponse)
	err := c.cc.Invoke(ctx, "/steward.v3.ContractCall/Schedule", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// ContractCallServer is the server API for ContractCall service.
// All implementations must embed UnimplementedContractCallServer
// for forward compatibility
type ContractCallServer interface {
	// Handles scheduled contract call submission
	Schedule(context.Context, *ScheduleRequest) (*ScheduleResponse, error)
	mustEmbedUnimplementedContractCallServer()
}

// UnimplementedContractCallServer must be embedded to have forward compatible implementations.
type UnimplementedContractCallServer struct {
}

func (UnimplementedContractCallServer) Schedule(context.Context, *ScheduleRequest) (*ScheduleResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Schedule not implemented")
}
func (UnimplementedContractCallServer) mustEmbedUnimplementedContractCallServer() {}

// UnsafeContractCallServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to ContractCallServer will
// result in compilation errors.
type UnsafeContractCallServer interface {
	mustEmbedUnimplementedContractCallServer()
}

func RegisterContractCallServer(s grpc.ServiceRegistrar, srv ContractCallServer) {
	s.RegisterService(&ContractCall_ServiceDesc, srv)
}

func _ContractCall_Schedule_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ScheduleRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ContractCallServer).Schedule(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v3.ContractCall/Schedule",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ContractCallServer).Schedule(ctx, req.(*ScheduleRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// ContractCall_ServiceDesc is the grpc.ServiceDesc for ContractCall service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var ContractCall_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v3.ContractCall",
	HandlerType: (*ContractCallServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Schedule",
			Handler:    _ContractCall_Schedule_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}

// SimulateContractCallClient is the client API for SimulateContractCall service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type SimulateContractCallClient interface {
	// Handles simulated contract call submission
	Simulate(ctx context.Context, in *SimulateRequest, opts ...grpc.CallOption) (*SimulateResponse, error)
}

type simulateContractCallClient struct {
	cc grpc.ClientConnInterface
}

func NewSimulateContractCallClient(cc grpc.ClientConnInterface) SimulateContractCallClient {
	return &simulateContractCallClient{cc}
}

func (c *simulateContractCallClient) Simulate(ctx context.Context, in *SimulateRequest, opts ...grpc.CallOption) (*SimulateResponse, error) {
	out := new(SimulateResponse)
	err := c.cc.Invoke(ctx, "/steward.v3.SimulateContractCall/Simulate", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// SimulateContractCallServer is the server API for SimulateContractCall service.
// All implementations must embed UnimplementedSimulateContractCallServer
// for forward compatibility
type SimulateContractCallServer interface {
	// Handles simulated contract call submission
	Simulate(context.Context, *SimulateRequest) (*SimulateResponse, error)
	mustEmbedUnimplementedSimulateContractCallServer()
}

// UnimplementedSimulateContractCallServer must be embedded to have forward compatible implementations.
type UnimplementedSimulateContractCallServer struct {
}

func (UnimplementedSimulateContractCallServer) Simulate(context.Context, *SimulateRequest) (*SimulateResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Simulate not implemented")
}
func (UnimplementedSimulateContractCallServer) mustEmbedUnimplementedSimulateContractCallServer() {}

// UnsafeSimulateContractCallServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to SimulateContractCallServer will
// result in compilation errors.
type UnsafeSimulateContractCallServer interface {
	mustEmbedUnimplementedSimulateContractCallServer()
}

func RegisterSimulateContractCallServer(s grpc.ServiceRegistrar, srv SimulateContractCallServer) {
	s.RegisterService(&SimulateContractCall_ServiceDesc, srv)
}

func _SimulateContractCall_Simulate_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SimulateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SimulateContractCallServer).Simulate(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v3.SimulateContractCall/Simulate",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(SimulateContractCallServer).Simulate(ctx, req.(*SimulateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// SimulateContractCall_ServiceDesc is the grpc.ServiceDesc for SimulateContractCall service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var SimulateContractCall_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v3.SimulateContractCall",
	HandlerType: (*SimulateContractCallServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Simulate",
			Handler:    _SimulateContractCall_Simulate_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}
