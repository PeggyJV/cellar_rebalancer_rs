// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v4.25.1
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
	// Handles simple contract call submission
	Submit(ctx context.Context, in *SubmitRequest, opts ...grpc.CallOption) (*SubmitResponse, error)
}

type contractCallClient struct {
	cc grpc.ClientConnInterface
}

func NewContractCallClient(cc grpc.ClientConnInterface) ContractCallClient {
	return &contractCallClient{cc}
}

func (c *contractCallClient) Submit(ctx context.Context, in *SubmitRequest, opts ...grpc.CallOption) (*SubmitResponse, error) {
	out := new(SubmitResponse)
	err := c.cc.Invoke(ctx, "/steward.v3.ContractCall/Submit", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// ContractCallServer is the server API for ContractCall service.
// All implementations must embed UnimplementedContractCallServer
// for forward compatibility
type ContractCallServer interface {
	// Handles simple contract call submission
	Submit(context.Context, *SubmitRequest) (*SubmitResponse, error)
	mustEmbedUnimplementedContractCallServer()
}

// UnimplementedContractCallServer must be embedded to have forward compatible implementations.
type UnimplementedContractCallServer struct {
}

func (UnimplementedContractCallServer) Submit(context.Context, *SubmitRequest) (*SubmitResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Submit not implemented")
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

func _ContractCall_Submit_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SubmitRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ContractCallServer).Submit(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v3.ContractCall/Submit",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ContractCallServer).Submit(ctx, req.(*SubmitRequest))
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
			MethodName: "Submit",
			Handler:    _ContractCall_Submit_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}

// StatusClient is the client API for Status service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type StatusClient interface {
	Version(ctx context.Context, in *VersionRequest, opts ...grpc.CallOption) (*VersionResponse, error)
}

type statusClient struct {
	cc grpc.ClientConnInterface
}

func NewStatusClient(cc grpc.ClientConnInterface) StatusClient {
	return &statusClient{cc}
}

func (c *statusClient) Version(ctx context.Context, in *VersionRequest, opts ...grpc.CallOption) (*VersionResponse, error) {
	out := new(VersionResponse)
	err := c.cc.Invoke(ctx, "/steward.v3.Status/Version", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// StatusServer is the server API for Status service.
// All implementations must embed UnimplementedStatusServer
// for forward compatibility
type StatusServer interface {
	Version(context.Context, *VersionRequest) (*VersionResponse, error)
	mustEmbedUnimplementedStatusServer()
}

// UnimplementedStatusServer must be embedded to have forward compatible implementations.
type UnimplementedStatusServer struct {
}

func (UnimplementedStatusServer) Version(context.Context, *VersionRequest) (*VersionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Version not implemented")
}
func (UnimplementedStatusServer) mustEmbedUnimplementedStatusServer() {}

// UnsafeStatusServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to StatusServer will
// result in compilation errors.
type UnsafeStatusServer interface {
	mustEmbedUnimplementedStatusServer()
}

func RegisterStatusServer(s grpc.ServiceRegistrar, srv StatusServer) {
	s.RegisterService(&Status_ServiceDesc, srv)
}

func _Status_Version_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(VersionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StatusServer).Version(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v3.Status/Version",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(StatusServer).Version(ctx, req.(*VersionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Status_ServiceDesc is the grpc.ServiceDesc for Status service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Status_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v3.Status",
	HandlerType: (*StatusServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Version",
			Handler:    _Status_Version_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}
