from django.urls import path
from . import views

urlpatterns = [
    path('', views.list_devices, name='list_devices'),
    path('<int:pk>/', views.device_detail, name='device_detail'),
    path('<int:pk>/services/', views.device_services, name='device_services'),
    path('<int:pk>/software/', views.device_software, name='device_software'),
]
